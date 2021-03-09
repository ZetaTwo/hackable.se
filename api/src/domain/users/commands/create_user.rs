/*! Contains the `CreateUserCommand` type. */

use auto_impl::auto_impl;

use crate::domain::{
    users::{
        User,
        UserId,
        UserStore,

        Username,
        Email,
    },
    error::{
        err_msg,
        Error,
    },
    Resolver,
};

pub type Result = ::std::result::Result<(), Error>;

/** Input for a `CreateUserCommand`. */
#[derive(Clone, Deserialize)]
pub struct CreateUser {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
}

/** Create a User. */
#[auto_impl(FnMut)]
pub trait CreateUserCommand {
    fn create_user(&mut self, command: CreateUser) -> Result;
}

/** Default implementation for a `CreateUserCommand`. */
pub(in crate::domain) fn create_user_command(
    store: impl UserStore,
) -> impl CreateUserCommand {
    move |command: CreateUser| {
        debug!("creating user `{}`", command.id);

        let User = {
            if store.get_user(command.id)?.is_some() {
                err!("user `{}` already exists", command.id)?
            } else {
                User::new(
                    command.id,
                    command.username,
                    command.email,
                    false
                )?
            }
        };

        store.set_User(User)?;

        info!("user `{}` created", command.id);

        Ok(())
    }
}

impl Resolver {
    pub fn create_user_command(&self) -> impl CreateUserCommand {
        let store = self.Users().user_store();

        create_user_command(store)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Users::{
        model::store::in_memory_store,
        *,
    };

    use super::*;

    #[test]
    fn err_if_already_exists() {
        let store = in_memory_store();

        let create = CreateUser {
            id: UserId::new(),
        };

        let mut cmd = create_user_command(&store);

        cmd.create_user(create.clone()).unwrap();

        assert!(cmd.create_user(create).is_err());
    }
}
