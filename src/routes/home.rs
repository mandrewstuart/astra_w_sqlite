use astra::{Body, Request, Response};

// GET '/'
pub fn home(_: Request) -> Response {
    Response::new(Body::new("Welcome!"))
}