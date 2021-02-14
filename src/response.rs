use aws_lambda_events::encodings::Body;
use netlify_lambda_http::IntoResponse;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub struct Response {
    pub bytes: Vec<u8>,
}

impl Response {
    // pub fn into_lambda(
    //     self: Response,
    // ) -> Result<netlify_lambda_http::Response<aws_lambda_events::encodings::Body>, String> {
    //     Ok(netlify_lambda_http::Response::builder()
    //         .status(200)
    //         // TODO: need to change protobuf type
    //         .header("Content-Encoding", "application/protobuf")
    //         .body(aws_lambda_events::encodings::Body::from(self.bytes))
    //         .expect("failed to render response"))
    // }

    pub fn into_hyper(self: Response) -> Result<hyper::Response<hyper::Body>, Error> {
        Ok(hyper::Response::builder()
            .status(200)
            // TODO: need to change protobuf type
            // .header("Content-Encoding", "application/protobuf")
            .body(hyper::Body::from(self.bytes))
            .expect("failed to render response"))
    }
}

impl From<&str> for Response {
    fn from(s: &str) -> Self {
        Response { bytes: s.into() }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> netlify_lambda_http::Response<Body> {
        let body = Body::Binary(self.bytes);
        netlify_lambda_http::Response::new(body)
    }
}
