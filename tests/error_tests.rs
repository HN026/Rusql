use rusql::error::RUSQLError;

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_not_implemented_error() {
        let error = RUSQLError::NotImplemented("Feature XYZ".to_string());
        assert_eq!(error.to_string(), "Not Implemented error: Feature XYZ");
    }

    #[test]
    fn test_general_error() {
        let error = RUSQLError::General("Something went wrong".to_string());
        assert_eq!(error.to_string(), "General error: Something went wrong");
    }

    #[test]
    fn test_internal_error() {
        let error = RUSQLError::Internal("Internal failure".to_string());
        assert_eq!(error.to_string(), "Internal error: Internal failure");
    }

    #[test]
    fn test_unknown_command_error() {
        let error = RUSQLError::UnknownCommand("INVALID COMMAND".to_string());
        assert_eq!(error.to_string(), "Unknown command error: INVALID COMMAND");
    }

    #[test]
    fn test_error_equality() {
        let error1 = RUSQLError::General("Test".to_string());
        let error2 = RUSQLError::General("Test".to_string());
        assert_eq!(error1, error2);

        let error3 = RUSQLError::General("Different".to_string());
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_error_debug() {
        let error = RUSQLError::NotImplemented("Debug test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("NotImplemented"));
        assert!(debug_str.contains("Debug test"));
    }
}
