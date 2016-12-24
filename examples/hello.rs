extern crate hayaku_http;
extern crate hayaku_simple_path;

use hayaku_http::{Http, Request, Response};
use hayaku_simple_path::Router;

use std::sync::Arc;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();

    let mut router = Router::new();
    router.add_route("/", Arc::new(home_handler));

    Http::new(router, ()).listen_and_serve(addr);
}

fn home_handler(_req: &Request, res: &mut Response, _ctx: &()) {
    res.body(b"hello, world!").unwrap();
}
