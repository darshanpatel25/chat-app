use actix_web::web::Data;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::ticket::{AssignTicket, NewTicket, Ticket, UpdateStatus};
use crate::utils::jwt::Claims;

pub async fn create_ticket(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    info: web::Json<NewTicket>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>();

    if claims.is_none() {
        return HttpResponse::Unauthorized().body("Missing Token!! Unauthorized");
    }

    let claims = claims.unwrap();
    let user_id = claims.sub;

    let result = sqlx::query(
        "INSERT INTO tickets (title, description, created_by, assigned_to) VALUES (?, ?, ?, ?)",
    )
    .bind(&info.title)
    .bind(&info.description)
    .bind(user_id)
    .bind(info.assigned_to) 
    .execute(&**pool)
    .await;

    if let Err(e) = result {
        eprintln!("DB Insert Error: {:?}", e);
        return HttpResponse::InternalServerError().body("Error inserting ticket");
    }

    let last_id = result.unwrap().last_insert_id();

    let ticket = sqlx::query_as::<_, Ticket>("SELECT * FROM tickets WHERE id = ?")
        .bind(last_id)
        .fetch_one(pool.as_ref())
        .await;

    match ticket {
        Ok(ticket) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Ticket Created Successfully",
            "success": true,
            "ticket": ticket
        })),
        Err(e) => {
            eprintln!("Error fetching ticket: {:?}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

pub async fn get_all_tickets(pool: web::Data<MySqlPool>)-> impl Responder{
    let result = sqlx::query_as::<_,Ticket>("SELECT * FROM tickets").fetch_all(pool.as_ref()).await;
    match result{
        Ok(tickets)=>{
            HttpResponse::Ok().json(tickets)
        }
        Err(e)=>{
            eprint!("Error: {:?}",e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }

    }
}

pub async fn update_ticket_status(
    pool: web::Data<MySqlPool>,
    info: web::Json<UpdateStatus>
)->impl Responder{
    let result = sqlx::query("UPDATE tickets SET status = ? WHERE id = ?")
        .bind(&info.status)
        .bind(info.ticket_id)
        .execute(pool.as_ref())
        .await;

    match result{
        Ok(_) => HttpResponse::Ok().body("Status updated"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update status")
    }
}

pub async fn assign_ticket(
    pool: web::Data<MySqlPool>,
    info: web::Json<AssignTicket>
)-> impl Responder{
    let result = sqlx::query("UPDATE tickets SET assigned_to = ?, status = ? WHERE id =?")
        .bind(info.developer_id)
        .bind("assigned")
        .bind(info.ticket_id)
        .execute(pool.as_ref())
        .await;

    match result{
        Ok(_)=>{
            HttpResponse::Ok().body("Ticket Assigned Successfully")
        }
        Err(e)=>{
            eprint!("Error : {:?}",e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
