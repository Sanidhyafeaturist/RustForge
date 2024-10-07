pub struct Lifecycle {
    pub is_running: bool,
}

impl Lifecycle {
    pub fn new() -> Self {
        Lifecycle { is_running: false }
    }

    pub fn start(&mut self) {
        self.is_running = true;
        println!("Application started.");
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        println!("Application stopped.");
    }
}
