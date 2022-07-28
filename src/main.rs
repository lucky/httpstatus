use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::str::FromStr;
use std::{convert::Infallible, net::SocketAddr};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();

    if path == "/" {
        return Ok(
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Body::from("Head to <a href=\"/200\">/200</a> for a 200 response, <a href=\"/404\">/404</a> for a 404, etc"))
                .unwrap(),
        );
    }

    match StatusCode::from_str(&path[1..]) {
        Ok(status) => Ok(Response::builder()
            .status(status)
            .body(Body::from(format!(
                "{} {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or("unknown status")
            )))
            .unwrap()),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Invalid status code"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let port = match std::env::var("LISTEN_PORT") {
        Ok(port) => port
            .parse()
            .unwrap_or_else(|_| panic!("Invalid port `{}`", port)),
        Err(_) => 3999,
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
