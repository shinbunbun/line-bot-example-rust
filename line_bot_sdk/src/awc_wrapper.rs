use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::error::Error;
use actix_web::web::Bytes;
use awc::SendClientRequest;
use futures::Future;

pub struct SendClientRequestFut<T> {
    fut: Pin<Box<dyn Future<Output = Result<T, Error>>>>,
}

impl<T> SendClientRequestFut<T>
where
    T: for<'a> serde::Deserialize<'a>,
{
    pub fn new(req: Result<SendClientRequest, Error>) -> Self {
        let fut = async move {
            let req = req?;
            let mut res = req.await.map_err(Error::AwcSendRequestError)?;
            let res_struct = res.json::<T>().await.map_err(Error::AwcJsonPayloadError)?;
            Ok(res_struct)
        };

        Self { fut: Box::pin(fut) }
    }
}

impl<T> Future for SendClientRequestFut<T>
where
    T: for<'a> serde::Deserialize<'a>,
{
    type Output = Result<T, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        this.fut.as_mut().poll(cx)
    }
}

pub struct SendClientRequestByteFut {
    fut: Pin<Box<dyn Future<Output = Result<Bytes, Error>>>>,
}

impl SendClientRequestByteFut {
    pub fn new(req: Result<SendClientRequest, Error>) -> Self {
        let fut = async move {
            let req = req?;
            let mut res = req.await.map_err(Error::AwcSendRequestError)?;
            res.body().await.map_err(Error::ActixWebPayloadError)
        };

        Self { fut: Box::pin(fut) }
    }
}

impl Future for SendClientRequestByteFut {
    type Output = Result<Bytes, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        this.fut.as_mut().poll(cx)
    }
}
