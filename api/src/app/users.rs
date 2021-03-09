/*! `/users` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::{
        err_msg,
        Error,
    },
    domain::{
        users::*,
        id::IdProvider,
        Resolver,
    },
};

/** `GET /users/<id>` */
#[get("/<id>")]
pub fn get(id: UserId, resolver: State<Resolver>) -> Result<Json<User>, Error> {
    Err(Error::Other(err_msg("not implemented")))
    /*let query = resolver.get_customer_with_orders_query();

    match query.get_customer_with_orders(GetCustomerWithOrders { id: id })? {
        Some(customer) => Ok(Json(customer)),
        None => Err(Error::NotFound(err_msg("customer not found"))),
    }*/
}

/** `PUT /users` */
#[put("/", format = "application/json")]
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
#[delete("/")]
pub fn delete(id: UserId, resolver: State<Resolver>) -> Result<Json<UserId>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}
