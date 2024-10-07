pub struct Auth;

impl Auth {
    pub fn authorize(&self, token: &str) -> bool {
        token == "valid_token"
    }
}
