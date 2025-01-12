use diesel::prelude::*;
use crate::models::user_model::{NewUser, Users};
use crate::schema::users;
use regex::Regex;


#[derive(Debug)]
pub enum ValidationError {
    InvalidEmail,
    EmptyField(String),
    WeakPassword,
}

#[allow(dead_code)]
pub fn validate_user(new_user: &NewUser) -> Result<(), ValidationError> {
    // Check for empty fields
    if new_user.first_name.trim().is_empty() {
        return Err(ValidationError::EmptyField("first_name".to_string()));
    }
    if new_user.last_name.trim().is_empty() {
        return Err(ValidationError::EmptyField("last_name".to_string()));
    }
    if new_user.username.trim().is_empty() {
        return Err(ValidationError::EmptyField("username".to_string()));
    }
    if new_user.email.trim().is_empty() {
        return Err(ValidationError::EmptyField("email".to_string()));
    }
    if new_user.user_password.trim().is_empty() {
        return Err(ValidationError::EmptyField("user_password".to_string()));
    }

    // Validate email format using regex
    let email_regex = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(new_user.email) {
        return Err(ValidationError::InvalidEmail);
    }

    // Validate password strength
    // Example: password must be at least 8 characters long
    if new_user.user_password.len() < 8 {
        return Err(ValidationError::WeakPassword);
    }

    Ok(())
}


#[allow(dead_code)]
pub fn create_new_user(
    conn: &mut PgConnection,
    new_user: NewUser,
) -> Result<Users, Box<dyn std::error::Error>> {
    // Validate the user data
    if let Err(validation_error) = validate_user(&new_user) {
        return Err(Box::new(validation_error));
    }

    // Insert into the database
    let inserted_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?;

    Ok(inserted_user)
}

// Implement `std::fmt::Display` for `ValidationError` for better error messages
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidEmail => write!(f, "Invalid email format"),
            ValidationError::EmptyField(field) => write!(f, "The field '{}' cannot be empty", field),
            ValidationError::WeakPassword => write!(f, "Password must be at least 8 characters long"),
        }
    }
}

// Implement `std::error::Error` for `ValidationError`
impl std::error::Error for ValidationError {}
