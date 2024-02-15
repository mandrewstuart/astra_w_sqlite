use astra::{Body, Request, Response};
use std::collections::HashMap;
type Params = HashMap<String, String>;

use super::model;

// GET '/user/:id'
pub fn get_user(req: Request) -> Response {
    model::tester();
    // Retreive route parameters from the the request extensions
    let params = req.extensions().get::<Params>().unwrap();

    // Get the 'id' from '/user/:id'
    let id = params.get("id").unwrap();

    Response::new(Body::new(format!("User #{id}")))
}
