use std::future::{ready, Ready};

use actix_web::body::BoxBody;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

pub struct Encryption;

impl<S> Transform<S, ServiceRequest> for Encryption
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = EncryptionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(EncryptionMiddleware { service }))
    }
}

pub struct EncryptionMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for EncryptionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let pay = req.take_payload();
        //TODO not important: decrypt
        req.set_payload(pay);
        let fut = self.service.call(req);

        Box::pin(async move {
            let res: ServiceResponse<BoxBody> = fut.await?;
            let res = res.map_body(|_, b| {
                //TODO not important: encrypt
                b
            });
            Ok(res)
        })
    }
}
