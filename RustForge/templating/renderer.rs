use std::fs::File;
use std::io::{self, Read};

pub struct Renderer;

impl Renderer {
    pub fn render(&self, template_path: &str) -> Result<String, io::Error> {
        let mut file = File::open(template_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
