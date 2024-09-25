// src/utils/auth.rs
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures::future::{ok, Ready, LocalBoxFuture};
use crate::services::auth_service::AuthService;
use uuid::Uuid;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|s| s.to_owned());

        if let Some(token) = token {
            match AuthService::verify_token(&token) {
                Ok(claims) => {
                    req.extensions_mut().insert(AuthGuard {
                        user_id: Uuid::parse_str(&claims.sub).unwrap(),
                    });
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                }
                Err(_) => Box::pin(async move {
                    Ok(req.into_response(
                        actix_web::HttpResponse::Unauthorized().finish().into_body(),
                    ))
                }),
            }
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    actix_web::HttpResponse::Unauthorized().finish().into_body(),
                ))
            })
        }
    }
}

pub struct AuthGuard {
    pub user_id: Uuid,
}
