use axum::body::Body;
use axum::response::Response;
use axum::routing::get;
use axum::{extract::ConnectInfo, http::StatusCode, routing::any, Router};
use hyper::Request;
// use hyper::{Body, Request, Response};
use std::{
    convert::Infallible,
    net::{IpAddr, SocketAddr},
};

async fn handle(client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match hyper_reverse_proxy::call(client_ip, "https://google.com/", req).await {
        Ok(response) => Ok(response),
        Err(error) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("error: {:?}", error).into())
            .unwrap()),
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

    async fn handler(
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        req: Request<Body>,
    ) -> Result<Response<Body>, Infallible> {
        handle(addr.ip(), req).await
    }

    let app = Router::new()
        .route("/", any(hello))
        .route("/test", any(handler));

    let server =
        axum::Server::bind(&baddr).serve(app.into_make_service_with_connect_info::<SocketAddr>());

    println!("Running server on {:?}", baddr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
