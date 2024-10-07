#[cfg(test)]
mod tests {
    use super::super::core::errors::FrameworkError;

    #[test]
    fn test_error_display() {
        let error = FrameworkError::NotFound("Resource".to_string());
        assert_eq!(format!("{}", error), "Not Found: Resource");
    }
}
