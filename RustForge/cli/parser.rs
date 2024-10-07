use std::env;

pub struct CLIParser;

impl CLIParser {
    pub fn parse_args(&self) -> Vec<String> {
        env::args().collect()
    }
}
