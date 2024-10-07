use my_rust_framework::core::config::Config;
use my_rust_framework::routing::router::Router;

fn main() {
    let config = Config::new("BasicApp", "0.1.0", 8080);
    println!("Running {} v{}", config.app_name, config.version);

    let mut router = Router::new();
    router.add_route("/home", |req| {
        println!("Handling home request: {}", req);
    });
    
    router.handle_request("/home");
}
