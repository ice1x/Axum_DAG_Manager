use crate::error::{AppError, Result};

pub fn validate_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Name cannot be empty".to_string(),
        ));
    }
    if name.len() > 255 {
        return Err(AppError::ValidationError(
            "Name cannot exceed 255 characters".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_label(label: &str) -> Result<()> {
    if label.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Label cannot be empty".to_string(),
        ));
    }
    if label.len() > 255 {
        return Err(AppError::ValidationError(
            "Label cannot exceed 255 characters".to_string(),
        ));
    }
    Ok(())
}
