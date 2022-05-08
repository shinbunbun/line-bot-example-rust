use crate::config;
use actix_http::{body::EitherBody, error::PayloadError, header, HttpMessage};
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::Encoding,
    web::{self, Bytes, BytesMut},
    Either, Error, FromRequest, HttpRequest, HttpResponse,
};
use futures::{executor::block_on, Future, FutureExt};
use futures_util::{
    future::{self, LocalBoxFuture},
    StreamExt,
};
use hex_literal::hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};
const DEFAULT_CONFIG_LIMIT: usize = 262_144;
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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    dev::forward_ready!(service);

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        println!("{}", 1);
        /* let signature = request.headers().get("x-line-signature");

        if signature.is_none() {
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::BadRequest()
                .body("x-line-signature not found")
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }
        let signature = signature.unwrap(); */

        println!("{}", 2);
        let (req, payload) = request.into_parts();

        /* async move {
            println!("{}", 3);
            let mut stream = payload;
            let mut body = BytesMut::new();
            while let Some(chunk) = stream.next().await {
                println!("{}", 4);
                body.extend_from_slice(&chunk.unwrap());
            }
            println!("{}", 5);
            println!("request body: {:?}", body);

            /* let svc_res = svc.call(req).await?;

            let mut svc_res = enable_response_buffering(&wrapper, svc_res);

            let mut stream = svc_res.take_body();
            let mut body = BytesMut::new();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk.unwrap());
            }
            let svc_res = svc_res.map_body(|_, _| stream);
            println!("response body: {:?}", body); */

            /* self.service
            .call(request)
            .await
            .map(ServiceResponse::map_into_left_body) */
            let request = ServiceRequest::from_request(req);

            let res = self.service.call(request);

            // forwarded responses map to "left" body

            res.await.map(ServiceResponse::map_into_left_body)
        }
        .boxed_local() */

        /* let mut length = None;
        let mut err = None;
        if let Some(l) = req.headers().get(&header::CONTENT_LENGTH) {
            match l.to_str() {
                Ok(s) => match s.parse::<usize>() {
                    Ok(l) => {
                        if l > DEFAULT_CONFIG_LIMIT {
                            err = Some(PayloadError::Overflow);
                        }
                        length = Some(l)
                    }
                    Err(_) => err = Some(PayloadError::UnknownLength),
                },
                Err(_) => err = Some(PayloadError::UnknownLength),
            }
        }

        let stream = payload.take(); */
        /* println!("{}", 3);
        let body = block_on(actix_web::web::Bytes::from_request(&request, &mut payload)).unwrap();
        println!("{}", 4);
        println!("body: {}", String::from_utf8(body.to_vec()).unwrap());
        println!("{}", 5); */

        // let payload = request.into_parts().1;

        /* let mut bytes = web::BytesMut::new();
        while let Some(item) = block_on(payload.next()) {
            println!("{}", 3);
            println!("{:?}", bytes);
            bytes.extend_from_slice(&item.unwrap());
        } */
        /*
        async fn temp(mut bytes: BytesMut, mut payload: actix_http::Payload) {
            println!("{}", 2);
            payload.next().await.unwrap();
            /* while let Some(item) = payload.next().await {
                println!("{}", 3);
                println!("{:?}", bytes);
                bytes.extend_from_slice(&item.unwrap());
            } */
            println!("{}", 4);
            let body = String::from_utf8(bytes.to_vec()).unwrap();
            println!("body: {}", body);
        }
        block_on(temp(bytes, payload));
        println!("{}", 5); */

        /* payload.map(|item| {
            let body = item.unwrap();
            let body = String::from_utf8(body.to_vec()).unwrap();
            println!("body: {}", body);
        }); */
        /* payload.for_each(|item| {
            let body = item.unwrap();
            let body = String::from_utf8(body.to_vec()).unwrap();
            println!("body: {}", body);
            /* let body = body.to_vec();
            let body = body.as_slice(); */
            /*
            let key = config::LINE_CHANNEL_SECRET.as_bytes();
            let mut mac = Hmac::<Sha256>::new_varkey(key).unwrap();
            mac.update(body);
            let digest = mac.finalize();
            let signature = hex::encode(digest.into_bytes());

            if signature != signature {
                panic!("signature not match");
            } */
            future::ready(())
        }); */

        /* let secret = config::get_secret();

        if secret.is_err() {
            // let (request, _pl) = request.into_parts();
            let response = HttpResponse::InternalServerError()
                .body("")
                .map_into_right_body();
            println!("secret error: {:?}", secret.err().unwrap());

            return Ok(ServiceResponse::new(request, response));
        }
        let secret = secret.unwrap();

        type HmacSha256 = Hmac<Sha256>;
        let mac = HmacSha256::new_from_slice(secret.as_bytes());
        if mac.is_err() {
            // let (request, _pl) = request.into_parts();
            let response = HttpResponse::InternalServerError()
                .body("")
                .map_into_right_body();
            println!("{:?}", mac.err().unwrap());

            return Ok(ServiceResponse::new(request, response));
        }
        let mut mac = mac.unwrap();
        // mac.update();

        let result = mac.finalize();

        let code_bytes = result.into_bytes();
        let expected = hex!(
            "
            97d2a569059bbcd8ead4444ff99071f4
            c01d005bcefe0d3567e1be628e5fdcd9
        "
        );
        assert_eq!(code_bytes[..], expected[..]); */

        /* let request = ServiceRequest::from_request(req);

        let res = self.service.call(request);

        // forwarded responses map to "left" body

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) }) */
    }
}
