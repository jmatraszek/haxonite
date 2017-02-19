use iron::{Handler, Request, Response, IronResult};
use iron::status;
use std::{thread, time};
use utils;

#[derive(Debug)]
pub struct SingleResponse {
    response_path: String,
    headers: Vec<Vec<String>>,
    status: u16,
    delay: Option<u64>,
}

impl<'a> SingleResponse {
    pub fn new(status: u16, headers: Vec<String>, response_path: String, delay: Option<u64>) -> SingleResponse {

        let headers_vec = headers.iter()
            .map(
                |header| header.split(": ").map(
                    // TODO: This is quite stupid and could be done with &str,
                    // but I decided to use String to not fight borrow checker for now...
                    |hdr| hdr.to_string()
                ).collect()
            )
            .collect();

        SingleResponse {
            status: status,
            response_path: response_path,
            headers: headers_vec,
            delay: delay,
        }
    }
}

impl Handler for SingleResponse {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut response = Response::with((status::Status::from_u16(self.status), utils::response_body(self.response_path.clone())));
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
