extern crate random_choice;

use iron::{Handler, Request, Response, IronResult};
use handlers::single_response::SingleResponse;
use self::random_choice::random_choice;

#[derive(Debug)]
pub struct RandomResponse {
    response_handlers: Vec<SingleResponse>,
    weights: Vec<f32>,
}

impl RandomResponse {
    pub fn new() -> RandomResponse {
        RandomResponse {
            response_handlers: vec![],
            weights: vec![],
        }
    }

    pub fn add_handler(&mut self, weight: u32, handler: SingleResponse) {
        self.response_handlers.push(handler);
        self.weights.push(weight as f32)
    }
}

impl Handler for RandomResponse {
    fn handle(&self, r: &mut Request) -> IronResult<Response> {
        match random_choice()
            .random_choice_f32(self.response_handlers.as_slice(),
                               self.weights.as_slice(),
                               1)
            .first() {
            Some(handler) => handler.handle(r),
            None => self.response_handlers.first().unwrap().handle(r),
        }
    }
}
