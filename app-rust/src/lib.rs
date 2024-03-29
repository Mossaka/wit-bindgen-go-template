use bindings::handler::*;

struct Handler {}

impl bindings::handler::Handler for Handler {
    fn handle_http(req: Request) -> Result<Response, HttpError> {
        for header in req.headers.iter() {
            println!("{}: {}", header.0, header.1);
        }

        for arg in req.params.iter() {
            println!("{}: {}", arg.0, arg.1);
        }

        let response = Response {
            status: 200,
            headers: None,
            body: Some(b"hello world!".to_vec()),
        };
        Ok(response)
    }
}

bindings::export!(Handler);
