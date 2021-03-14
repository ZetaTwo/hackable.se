/*! `/users` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::{err_msg, Error},
    db::DbConn,
    domain::{users::*, Resolver},
};

/** `GET /users/` */
#[get("/")]
pub fn list(connection: DbConn, resolver: State<Resolver>) -> Result<Json<UserPublicInfo>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `GET /users/<id>` */
#[get("/<user_id>")]
pub fn get(
    connection: DbConn,
    user_id: UserId,
    resolver: State<Resolver>,
) -> Result<Json<UserPublicInfo>, Error> {
    //Err(Error::Other(err_msg("not implemented")))
    /*let query = resolver.get_customer_with_orders_query();

    match query.get_customer_with_orders(GetCustomerWithOrders { id: id })? {
        Some(customer) => Ok(Json(customer)),
        None => Err(Error::NotFound(err_msg("customer not found"))),
    }*/

    use crate::schema::users::dsl::*;

    // TODO: Select and solves/score as well

    let query = resolver.get_user_query(connection); //let query = resolver.get_user_with_solves_query();

    match query.get_user(GetUser { id: user_id })? {
        Err(_) => Err(NotFound(format!("User {} not found", user_id.to_string()))),
        Ok(user) => Ok(Json(UserPublicInfo {
            id: user.id,
            username: user.username,
        })),
    }
}

/** `PUT /users` */
#[put("/<id>", format = "application/json")]
pub fn update(id: UserId, resolver: State<Resolver>) -> Result<Json<User>, Error> {
    Err(Error::Other(err_msg("not implemented")))
    /*let id_provider = resolver.customer_id_provider();
    let mut command = resolver.create_customer_command();

    let id = id_provider.id()?;

    command.create_customer(CreateCustomer { id: id })?;

    Ok(Json(id))*/
}

/** `POST /users` */
#[post("/", format = "application/json")]
pub fn create(resolver: State<Resolver>) -> Result<Json<User>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `DELETE /users/<id>` */
#[delete("/<id>")]
pub fn delete(id: UserId, resolver: State<Resolver>) -> Result<Json<UserId>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}
