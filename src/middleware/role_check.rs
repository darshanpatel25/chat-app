use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, forward_ready}, Error, HttpMessage};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::{env, future::Future, pin::Pin, rc::Rc};
use crate::utils::jwt::Claims;

pub struct RoleCheck {
    allowed_roles: Vec<String>,
}

impl RoleCheck {
    pub fn new(allowed_roles: Vec<&str>) -> Self {
        RoleCheck {
            allowed_roles: allowed_roles.into_iter().map(String::from).collect(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RoleCheck
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RoleCheckMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RoleCheckMiddleware {
            service: Rc::new(service),
            allowed_roles: self.allowed_roles.clone(),
        })
    }
}

pub struct RoleCheckMiddleware<S> {
    service: Rc<S>,
    allowed_roles: Vec<String>,
}

impl<S, B> actix_service::Service<ServiceRequest> for RoleCheckMiddleware<S>
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let allowed_roles = self.allowed_roles.clone();

        Box::pin(async move {
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid token"))?;

            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let decoded = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            )
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

            let user_role = decoded.claims.role.clone();

            println!("{user_role},{:?}",allowed_roles);
            if !allowed_roles.contains(&user_role) {
                return Err(actix_web::error::ErrorForbidden("Insufficient permissions"));
            }

            req.extensions_mut().insert(decoded.claims);

            service.call(req).await
        })
    }
}
