use std::convert::TryFrom;

const USERNAME_MIN_LENGTH: usize = 5;

#[derive(Debug, Serialize, Deserialize)]
pub struct Username(String);

// TODO: Add display
// TODO: TryFrom validation
#[derive(Debug)]
pub enum UsernameValidationError {
    FormatError,
}

impl TryFrom<String> for Username {
    type Error = UsernameValidationError;

    fn try_from(username: String) -> Result<Self, Self::Error> {
        // TODO: Add username character validation

        if username.len() < USERNAME_MIN_LENGTH {
            Err(UsernameValidationError::FormatError)
        } else {
            Ok(Username(username))
        }
    }
}