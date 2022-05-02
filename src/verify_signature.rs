use actix_http::body::EitherBody;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct Verifier;

impl<S, B> Transform<S, ServiceRequest> for Verifier
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifierMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifierMiddleware { service }))
    }
}

pub struct VerifierMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for VerifierMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let signature = request.headers().get("x-line-signature");

        if signature.is_none() {
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::BadRequest()
                .body("x-line-signature not found")
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let signature = signature.unwrap();
        println!("{:?}", signature);

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
