extern crate hyper;

use std::env;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::header::USER_AGENT;

fn main() {
    const FAKE_CONTENT: &str = "Hello, Facebook bot!";
    const REAL_CONTENT: &str = "Hello, Alex Jones!";
    let port : u16;

    match env::var("PORT") {
        Ok(p) => port = p.parse::<u16>().unwrap(),
        Err(e) => port = 3000,
    };

    fn hello_world(req: Request<Body>) -> Response<Body> {
        if req.headers()[USER_AGENT].to_str().unwrap().contains("facebookexternalhit/1.1") {
            return Response::new(Body::from(FAKE_CONTENT));
        } else {
            return Response::new(Body::from(REAL_CONTENT));
        }
    }

    let addr = ([127, 0, 0, 1], port).into();

    let new_svc = || {
        service_fn_ok(hello_world)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
