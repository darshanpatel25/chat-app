use actix_web::{post,web,HttpResponse,Responder,HttpRequest,HttpMessage};
use bcrypt::{hash, verify,DEFAULT_COST};
use sqlx::MySqlPool;
use crate::utils::jwt::Claims;

use crate::models::users::{LoginUser,RegisterUser,User};
use crate::utils::jwt::create_jwt;


#[post("/register")]
pub async fn register_user(
    pool: web::Data<MySqlPool>,
    info: web::Json<RegisterUser>,
) -> impl Responder {
    
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password,role, created_at FROM users WHERE email = ?"
    )
    .bind(&info.email)
    .fetch_optional(&**pool)
    .await;

    match existing_user {
        Ok(Some(_)) => {
            HttpResponse::Conflict().json(serde_json::json!({
                "error": "User with this email already exists"
            }))
        }
        Ok(None) => {
            // Hash password
            let hashed_pwd = match hash(&info.password, DEFAULT_COST) {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Failed to hash password: {:?}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to hash password"
                    }));
                }
            };

            // Insert user and get the inserted ID
            let result = sqlx::query(
                "INSERT INTO users (name, email, password) VALUES (?, ?, ?)"
            )
            .bind(&info.name)
            .bind(&info.email)
            .bind(&hashed_pwd)
            .execute(&**pool)
            .await;

            match result {
                Ok(insert_result) => {
                    // Retrieve the inserted user using LAST_INSERT_ID()
                    let user = sqlx::query_as::<_, User>(
                        "SELECT id, name, email, password,role, created_at FROM users WHERE email = ?"
                    )
                    .bind(&info.email)
                    .fetch_one(&**pool) 
                    .await;

                    match user {
                        Ok(user) => HttpResponse::Ok().json(user),
                        Err(e) => {
                            eprintln!("Failed to fetch inserted user: {:?}", e);
                            HttpResponse::InternalServerError().json(serde_json::json!({
                                "error": "Failed to fetch inserted user"
                            }))
                        }
                    }
                }
                Err(sqlx::Error::Database(db_err)) if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation => {
                    HttpResponse::Conflict().json(serde_json::json!({
                        "error": "User with this email already exists"
                    }))
                }
                Err(e) => {
                    eprintln!("Failed to register user: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("Failed to register user: {:?}", e)
                    }))
                }
            }
        }
        Err(e) => {
            eprintln!("Database error while checking email: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {:?}", e)
            }))
        }
    }
}

#[post("/login")]
pub async fn login_user(
    pool:web::Data<MySqlPool>,
    info: web::Json<LoginUser>
)->impl Responder{
    let user = sqlx::query_as::<_,User>("SELECT * FROM users WHERE email = ?")
        .bind(&info.email)
        .fetch_optional(pool.as_ref())
        .await;

    match user{
        Ok(Some(user))=>{
            let valid = verify(&info.password, &user.password).unwrap();
            if valid{
                let token = create_jwt(user.id, &user.role);
                HttpResponse::Ok().json(serde_json::json!({"Token":token}))
            }
            else{
                HttpResponse::Unauthorized().body("Invalid Username or Password")
            }
        }
        Ok(None)=>{
            HttpResponse::NotFound().body("Invalid Username or Password")
        }
        Err(e)=>{
            HttpResponse::InternalServerError().body("Internal Srevre Error")
        }
    }
}


#[actix_web::get("/protected")]
pub async fn protected_test(req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        HttpResponse::Ok().body(format!("Hello {}, role: {}", claims.sub, claims.role))
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}
