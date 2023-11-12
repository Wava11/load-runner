use std::collections::HashMap;

use crate::request::{AuthenticatedRequest, Request};

pub struct Client {
    requests: Vec<AuthenticatedRequest>,
}

impl Client {
    pub fn new<'a>(cookies: HashMap<String, String>, requests: &Vec<&'a Request>) -> Self {
        let cookies_header = cookies
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .fold("".into(), |acc, curr| format!("{} ;{}", acc, curr));
        let mut headers = HashMap::<String, String>::new();
        headers.insert("Cookie".into(), cookies_header);

        Self {
            requests: requests
                .into_iter()
                .map(|request| request.authenticate(&headers))
                .collect(),
        }
    }

    pub fn start() {}
}
