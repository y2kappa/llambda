use std::collections::HashMap;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
#[derive(Debug)]
pub struct GetRequest {
    pub path: String,
    pub parameters: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct PostRequest {
    pub path: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum Request {
    Get(GetRequest),
    Post(PostRequest),
}

impl Request {
    pub async fn from_lambda(req: netlify_lambda_http::Request) -> Result<Request, Error> {
        match req.method() {
            &http::Method::GET => Ok(Request::Get(utils::get_request_to_raw(&req))),
            &http::Method::POST => match utils::post_request_to_raw(req) {
                Ok(post) => Ok(Request::Post(post)),
                Err(err) => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err,
                ))),
            },
            _ => {
                println!("Unmatched request type");
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unmatched request type!",
                )))
            }
        }
    }

    pub async fn from_hyper(req: hyper::Request<hyper::Body>) -> Result<Request, Error> {
        match req.method() {
            &http::Method::GET => {
                let path = req.uri().path().to_owned();

                let parameters = match req.uri().query() {
                    Some(parameters) => {
                        let parameters: HashMap<String, String> =
                            url::form_urlencoded::parse(parameters.as_bytes())
                                .into_owned()
                                .collect();
                        let parameters: HashMap<String, Vec<String>> =
                            parameters.into_iter().map(|(k, v)| (k, vec![v])).collect();
                        parameters
                    }
                    None => HashMap::new(),
                };

                Ok(Request::Get(GetRequest { path, parameters }))
            }
            &http::Method::POST => {
                let path = req.uri().path().to_owned();
                let bytes = hyper::body::to_bytes(req.into_body())
                    .await
                    .unwrap()
                    .to_vec();

                Ok(Request::Post(PostRequest { path, bytes }))
            }
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Other method types are not supported",
            ))),
        }
    }
}

mod utils {
    use super::*;

    pub fn query_to_hashmap(query: &netlify_lambda_http::StrMap) -> HashMap<String, Vec<String>> {
        let mut hash = HashMap::new();
        for item in query.iter() {
            if let Some(values) = query.get_all(item.0) {
                hash.insert(
                    item.0.to_string(),
                    values.iter().map(|x| x.to_string()).collect(),
                );
            } else {
                hash.insert(item.0.to_string(), vec![]);
            }
        }

        hash
    }

    pub fn body_to_bytes(req: netlify_lambda_http::Request) -> Result<Vec<u8>, &'static str> {
        use aws_lambda_events::encodings::Body;
        let request = match req.into_body() {
            Body::Binary(v) => Ok(v),
            Body::Text(s) => Ok(s.into_bytes()),
            _ => Err("not found"),
        };
        request
    }

    pub fn request_to_path(request: &netlify_lambda_http::Request) -> String {
        let uri = request.uri();
        let path = uri.path();
        path.to_owned()
    }

    pub fn get_request_to_raw(request: &netlify_lambda_http::Request) -> GetRequest {
        use netlify_lambda_http::ext::RequestExt;
        GetRequest {
            path: request_to_path(&request),
            parameters: query_to_hashmap(&request.query_string_parameters()),
        }
    }

    pub fn post_request_to_raw(
        request: netlify_lambda_http::Request,
    ) -> Result<PostRequest, &'static str> {
        let path = request_to_path(&request);
        let bytes = match body_to_bytes(request) {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err);
            }
        };
        let request = PostRequest { path, bytes };
        Ok(request)
    }
}
