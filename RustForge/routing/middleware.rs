use std::time::Instant;

pub struct Middleware {
    pub name: String,
    pub logging_enabled: bool,
    pub auth_required: bool,
}

impl Middleware {
    pub fn new(name: &str, logging_enabled: bool, auth_required: bool) -> Self {
        Middleware {
            name: name.to_string(),
            logging_enabled,
            auth_required,
        }
    }

    pub fn apply(&self, request: &str) {
        if self.logging_enabled {
            let start_time = Instant::now();
            println!("{}: Processing request '{}'", self.name, request);
            // Process the request here...
            let duration = start_time.elapsed();
            println!("{}: Finished processing '{}' in {:?}", self.name, request, duration);
        }
    }
}
