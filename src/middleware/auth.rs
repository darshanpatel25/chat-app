use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
    body::EitherBody,
};
use futures_util::future::{ready, Ready, LocalBoxFuture};
use futures_util::FutureExt;
use std::{rc::Rc, task::{Context, Poll}};
use crate::utils::jwt::verify_jwt;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareInner {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareInner<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        if let Some(claims) = verify_jwt(token) {
                            req.extensions_mut().insert(claims);
                            let res = service.call(req).await?;
                            return Ok(res.map_into_left_body());
                        }
                    }
                }
            }

            let res = req.into_response(
                HttpResponse::Unauthorized()
                    .body("Unauthorized")
                    .map_into_right_body(),
            );

            Ok(res)
        }
        .boxed_local()
    }
}
