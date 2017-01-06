use iron::{Handler, Request, Response, IronResult};
use handlers::single_response::SingleResponse;
use std::sync::{Arc, Mutex};
use std::cmp;

#[derive(Debug)]
pub struct IteratingResponse {
    response_handlers: Vec<SingleResponse>,
    current: Arc<Mutex<Option<usize>>>,
    kind: &'static str,
}

impl IteratingResponse {
    pub fn new(kind: &'static str) -> IteratingResponse {
        IteratingResponse {
            response_handlers: vec![],
            current: Arc::new(Mutex::new(None)),
            kind: kind,
        }
    }

    pub fn add_handler(&mut self, handler: SingleResponse) {
        self.response_handlers.push(handler);
        let mut guard = self.current.lock().unwrap();
        match *guard {
            Some(_) => {}
            None => *guard = Some(0),
        }
    }
}

impl Handler for IteratingResponse {
    fn handle(&self, r: &mut Request) -> IronResult<Response> {
        let mut guard = self.current.lock().unwrap();
        let current = (*guard).unwrap(); // This is safe as validation on config will not allow an empty RoundrobinResponse
        let handler = &self.response_handlers[current];
        let len = self.response_handlers.len();
        *guard = match self.kind {
            "roundrobin" => Some((current + 1) % len),
            "chain" => Some(cmp::min(current + 1, len - 1)),
            _ => Some(0),
        };
        handler.handle(r)
    }
}
