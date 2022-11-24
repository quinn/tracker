use axum::{extract::ConnectInfo, http::StatusCode, routing::any, Router};
use hyper::{Body, Request, Response};
use std::net::IpAddr;
use std::{convert::Infallible, net::SocketAddr};

fn debug_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("hi {:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

async fn handle(client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri().path().starts_with("/target/first") {
        // will forward requests to port 13901
        match hyper_reverse_proxy::call(client_ip, "http://127.0.0.1:13901", req).await {
            Ok(response) => Ok(response),
            Err(_error) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()),
        }
    } else if req.uri().path().starts_with("/target/second") {
        // will forward requests to port 13902
        match hyper_reverse_proxy::call(client_ip, "http://127.0.0.1:13902", req).await {
            Ok(response) => Ok(response),
            Err(_error) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()),
        }
    } else {
        debug_request(req)
    }
}

#[tokio::main]
async fn main() {
    let bind_addr = "127.0.0.1:8000";
    let baddr: SocketAddr = bind_addr.parse().expect("Could not parse ip:port.");

    async fn hello(
        req: Request<Body>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> Result<Response<Body>, Infallible> {
        handle(addr.ip(), req).await
    }

    let app = Router::new().route("/", any(hello));

    let server =
        axum::Server::bind(&baddr).serve(app.into_make_service_with_connect_info::<SocketAddr>());

    println!("Running server on {:?}", baddr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
