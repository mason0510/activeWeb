use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, error, HttpMessage};
use actix_web::http::header::HeaderValue;
use futures::future::{LocalBoxFuture, ok};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = Middleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Middleware { service }))
    }
}

pub struct Middleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for Middleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let mut token =HeaderValue::from_str("").unwrap();
        //req 获取token
        {
            println!("Hi from start. You requested: {}", req.path());
            let x=req.headers().get("Authorization");
            match x{
                Some(x)=>{
                    token=x.clone();
                }
                None=>{
                    println!("no token");
                }
            }
            // token=x.clone();
        }
        //token验证
        //req.head_mut().insert("Authorization", HeaderValue::from_static("Bearer 12345678"));
        //token解析
        let fut = self.service.call(req);
        //获取request的header中的token
        if token.len()>0{
        Box::pin(async move {
                let res = fut.await?;
                println!("Hi from response");
                Ok(res)
        })
        }else {
            Box::pin(async move {
                Err(error::ErrorUnauthorized("err"))
            })
        }
    }
}
