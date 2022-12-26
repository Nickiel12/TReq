#[derive(Clone)]
pub enum METHODS {
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
}

#[derive(Clone)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: METHODS,
    pub headers: String,
    pub body: String,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: METHODS::GET,
            name: String::from("New Request"),
            url: String::new(),
            headers: String::new(),
            body: String::new(),
        }
    }
}

impl Request {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_method(&mut self, method: METHODS) {
        self.method = method;
    }

    pub fn set_headers(&mut self, headers: String) {
        self.headers = headers;
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}
