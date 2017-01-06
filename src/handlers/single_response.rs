use iron::{Handler, Request, Response, IronResult};
use iron::status;
use utils;

#[derive(Debug)]
pub struct SingleResponse {
    response_path: String,
    content_type: String,
    status: u16,
}

impl SingleResponse {
    pub fn new(status: u16, content_type: String, response_path: String) -> SingleResponse {
        SingleResponse {
            status: status,
            response_path: response_path,
            content_type: content_type,
        }
    }
}

impl Handler for SingleResponse {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut response = Response::with((status::Status::from_u16(self.status), utils::response_body(self.response_path.clone())));
        response.headers.set_raw("Content-Type", vec![self.content_type.clone().into_bytes()]);
        Ok(response)
    }
}
