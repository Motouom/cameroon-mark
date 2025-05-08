use validator::Validate;
use crate::errors::AppError;

// Validate a struct that implements the Validate trait
pub fn validate<T: Validate>(value: &T) -> Result<(), AppError> {
    value.validate().map_err(|e| {
        let error_message = e
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let error_messages: Vec<String> = errors
                    .iter()
                    .map(|error| error.message.clone().unwrap_or_else(|| "Invalid input".into()).to_string())
                    .collect();
                format!("{}: {}", field, error_messages.join(", "))
            })
            .collect::<Vec<String>>()
            .join("; ");
        
        AppError::validation(error_message)
    })
}
