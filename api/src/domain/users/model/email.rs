use std::convert::TryFrom;

const EMAIL_MIN_LENGTH: usize = 5;

//type EmailString = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Email(String);

// Add display
#[derive(Debug)]
pub enum EmailValidationError {
    FormatError,
}

impl TryFrom<String> for Email {
    type Error = EmailValidationError;

    fn try_from(email: String) -> Result<Self, Self::Error> {
        // TODO: Add email format validation

        if email.len() < EMAIL_MIN_LENGTH {
            Err(EmailValidationError::FormatError)
        } else {
            Ok(Email(email))
        }
    }
}
