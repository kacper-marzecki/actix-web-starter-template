// use crate::common::AppError;
// use actix_identity::{Identity, RequestIdentity};
// use actix_service::{Service, Transform};
// use actix_web::{dev::ServiceRequest, dev::ServiceResponse, error::ResponseError, Error};
// use futures::future::{ok, Ready};
// use futures::Future;
// use std::pin::Pin;
// use std::task::{Context, Poll};
// pub struct AuthGuard;

// impl<S, B> Transform<S> for AuthGuard
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = AuthGuardMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(AuthGuardMiddleware { service })
//     }
// }

// pub struct AuthGuardMiddleware<S> {
//     service: S,
// }

// impl<S, B> Service for AuthGuardMiddleware<S>
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Pin<Box<Ready<Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&mut self, req: ServiceRequest) -> Self::Future {
//         // println!("Hi from start. You requested: {}", req.path());
//         let identity = req.get_identity();
//         let fut = self.service.call(req);
//         // let error_response =
//         //     req.error_response().error_response());
//         Box::pin(async move {
//             let res = match identity {
//                 Some(_) => fut.await?,
//                 None => req.error_response(AppError::Forbidden(json!("Unauthorized"))),
//                 // None => AppError::Forbidden(json!("Unauthorized").error_response(),
//             };
//             Ok(res)
//         })
//     }
// }
