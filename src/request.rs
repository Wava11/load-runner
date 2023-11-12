use std::{collections::HashMap, future::Future};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Error, Method, RequestBuilder, Response,
};

pub struct Request {
    request_builder: RequestBuilder,
}

impl Request {
    pub fn new(url: String, method: Method) -> Self {
        Self {
            request_builder: Client::new().request(method, url),
        }
    }

    pub fn authenticate(&self, headers: &HashMap<String, String>) -> AuthenticatedRequest {
        let headers_map: HeaderMap = headers
            .into_iter()
            .map(|(key, value)| {
                (
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_bytes(value.as_bytes()).unwrap(),
                )
            })
            .collect();
        AuthenticatedRequest {
            request_builder: self
                .request_builder
                .try_clone()
                .unwrap()
                .headers(headers_map),
        }
    }
}

pub struct AuthenticatedRequest {
    request_builder: RequestBuilder,
}

impl AuthenticatedRequest {
    pub async fn invoke(&self) -> impl Future<Output = Result<Response, Error>> {
        self.request_builder.try_clone().unwrap().send()
    }
}
