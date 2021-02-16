
pub fn from_lambda(
    lambda_response: http::Response<aws_lambda_events::encodings::Body>,
) -> hyper::Response<hyper::Body> {
    let body = match lambda_response.body().clone() {
        aws_lambda_events::encodings::Body::Empty => hyper::Body::empty(),
        aws_lambda_events::encodings::Body::Text(txt) => hyper::Body::from(txt.clone()),
        aws_lambda_events::encodings::Body::Binary(txt) => hyper::Body::from(txt.clone()),
    };

    let mut headers = lambda_response.headers().clone();

    let mut builder = hyper::Response::builder();
    builder.headers_mut().replace(&mut headers);
    let builder = builder.status(lambda_response.status());
    let resp = builder.body(body).expect("Could not build response");

    println!("Response {:?}", resp);
    resp
}
