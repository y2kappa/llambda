# Netlify Rust local testing

Test your Netlify Rust function locally. No docker container necessary.



Full working example [here](https://github.com/y2kappa/llambda-example). I use this as my getting started template.

## Context

This library provides types and convertors to allow you to test your Netlify function locally using a hyper local server. The trick is to convert the lambda request and hyper request types into a common type, handle that separately, and return a common response type that converts back to the lambda response type or the hyper response type.

The lambda request parameters are a bit weird, I replicated them in new types provided in the `request` module. Even if they don't fully make sense to me, I wanted to stay true to what's happening in prod.


The common handler:

```rust
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

use aws_lambda_events::encodings::Body;
use http::Response;
use llambda::request::Request;

pub async fn handle(_: Request) -> Result<Response<Body>, Error> {
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Body::from("🦀 Hello, Netlify 🦀"))
        .expect("failed to render response");

    Ok(response)
}

```

The lambda function:
```rust
use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod handler;

#[lambda(http)]
#[tokio::main]
async fn main(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let req = llambda::request::Request::from_lambda(request).await?;
    handler::handle(req).await
}
```

The hyper server:
```rust
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod handler;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let addr = ([127, 0, 0, 1], 7878).into();
    let server = Server::bind(&addr).serve(make_svc);
    server.await?;
    Ok(())
}

async fn handle(request: Request<Body>) -> Result<Response<Body>, Error> {
    let req = llambda::request::Request::from_hyper(request).await?;
    let lambda_response = handler::handle(req).await?;
    let resp = llambda::response::from_lambda(lambda_response);
    Ok(resp)
}

```

## How to build

1. Create a `server.rs` and a `handler.rs` where the handler and server are implemented.
2. Add the binaries to `Cargo.toml` `
```toml
[[bin]]
name = "function"
path = "src/main.rs"

[[bin]]
name = "server"
path = "src/server.rs"
```
3. ` $ cargo build `
4. `./target/debug/server`