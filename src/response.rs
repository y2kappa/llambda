pub fn from_lambda(
    lambda_response: http::Response<aws_lambda_events::encodings::Body>,
) -> hyper::Response<hyper::Body> {
    // Convert AWS response to hyper response
    // Convert: status, headers, body

    // make builder and add status
    let mut builder = hyper::Response::builder().status(lambda_response.status());

    // copy headers
    let headers = lambda_response.headers().clone();
    for (key, value) in headers {
        if let Some(key) = key {
            builder = builder.header(key, value);
        }
    }

    // set body
    let body = match lambda_response.body().clone() {
        aws_lambda_events::encodings::Body::Empty => hyper::Body::empty(),
        aws_lambda_events::encodings::Body::Text(txt) => hyper::Body::from(txt.clone()),
        aws_lambda_events::encodings::Body::Binary(txt) => hyper::Body::from(txt.clone()),
    };
    builder.body(body).expect("Could not build response")
}
