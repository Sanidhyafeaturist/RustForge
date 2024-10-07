use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, fn(&str)>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, handler: fn(&str)) {
        self.routes.insert(path.to_string(), handler);
    }

    pub fn handle_request(&self, request: &str) {
        if let Some(handler) = self.routes.get(request) {
            handler(request);
        } else {
            eprintln!("404 Not Found for request '{}'", request);
        }
    }
}
