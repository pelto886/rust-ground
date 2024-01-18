use core::prelude::*;
use std::error::Error;

#[derive(Debug)]
pub struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Default, Clone)]
pub struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Requestbuilder::Default();
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url.insert(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method.insert(method.into());
        self
    }
    pub fn headers(mut self, headers: impl Into<String>) -> Self {
        self.headers.insert(headers.into());
        self
    }
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body.insert(body.into());
        self
    }

    pub fn build(self) -> Result<Request> {
        let Some(url) = self.url else {
            return Err(Error::Static("No URL"));
        }
    }

}