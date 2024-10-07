use my_rust_framework::core::config::Config;
use my_rust_framework::routing::router::Router;
use my_rust_framework::logging::logger::Logger;
use my_rust_framework::security::auth::Auth;

fn main() {
    // Load configuration
    let config = Config::new("AdvancedApp", "0.2.0", 8081);
    println!("Running {} v{} on port {}", config.app_name, config.version, config.port);

    // Set up logging
    let logger = Logger::new("app.log");

    // Initialize router
    let mut router = Router::new();
    
    // Define routes
    router.add_route("/home", |req| {
        println!("Handling home request: {}", req);
    });

    router.add_route("/secure", |req| {
        let auth = Auth;
        let token = "valid_token"; // Replace with actual token extraction logic
        if auth.authorize(token) {
            println!("Handling secure request: {}", req);
        } else {
            println!("Unauthorized access attempt for request: {}", req);
        }
    });

    // Simulate handling incoming requests
    let requests = vec!["/home", "/secure", "/unknown"];
    for request in requests {
        logger.log(&format!("Received request: {}", request));
        router.handle_request(request);
    }
}
