#[macro_use]
extern crate log;
extern crate hayaku_http;

use hayaku_http::{Handler, Request, RequestHandler, Response, Status};

#[derive(Clone)]
pub struct Router<T: Clone> {
    paths: Vec<(String, RequestHandler<T>)>,
    not_found: Option<RequestHandler<T>>,
}

impl<T: Clone> Router<T> {
    pub fn new() -> Self {
        Router {
            paths: Vec::new(),
            not_found: None,
        }
    }

    pub fn set_not_found_handler(&mut self, handle: RequestHandler<T>) {
        self.not_found = Some(handle);
    }

    pub fn add_route<S: Into<String>>(&mut self, route: S, handle: RequestHandler<T>) {
        let route = route.into();
        self.paths.push((route, handle));
    }
}

impl<T: Clone> Handler<T> for Router<T> {
    fn handler(&self, req: &Request, res: &mut Response, ctx: &T) {
        let path = &req.path();
        debug!("path: {}", path);
        for &(ref route, ref handle) in &self.paths {
            if path == route {
                return handle(req, res, ctx);
            }
        }

        if self.not_found.is_some() {
            let handle = self.not_found.clone().unwrap();
            handle(req, res, ctx);
        } else {
            res.status(Status::NotFound);
            let msg = String::from("404, path \"") + path + "\" not found :(";
            res.body(msg.as_bytes());

        }
    }
}
