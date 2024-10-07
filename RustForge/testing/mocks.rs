pub struct Mock;

impl Mock {
    pub fn new() -> Self {
        Mock
    }

    pub fn mock_response(&self) -> &'static str {
        "Mock response generated."
    }
}
