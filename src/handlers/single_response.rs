use iron::status;
use iron::{Handler, IronResult, Request, Response};
use std::{thread, time};
use utils;

#[derive(Debug)]
pub struct SingleResponse {
    response_path: String,
    headers: Vec<Vec<String>>,
    status: u16,
    delay: Option<u64>,
}

impl SingleResponse {
    pub fn new(status: u16, headers: Vec<Vec<String>>, response_path: String, delay: Option<u64>) -> SingleResponse {
        SingleResponse {
            status: status,
            response_path: response_path,
            headers: headers,
            delay: delay,
        }
    }
}

impl Handler for SingleResponse {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut response = Response::with((
            status::Status::from_u16(self.status),
            utils::response_body(self.response_path.clone()),
        ));
        for header in &self.headers {
            // FIXME: This will fail for headers in a wrong format (without ": ")
            response.headers.set_raw(header[0].clone(), vec![header[1].as_bytes().to_vec()]);
        }
        if let Some(delay) = self.delay {
            thread::sleep(time::Duration::from_secs(delay));
        };
        Ok(response)
    }
}
