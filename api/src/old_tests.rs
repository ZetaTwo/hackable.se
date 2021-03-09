use super::rocket;
use crate::db::run_db_migrations;
use crate::db::DbConn;
use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use rocket::Rocket;

fn test_rocket() -> rocket::Rocket {
    super::rocket_base()
}

#[test]
fn create_user() {
    let client = Client::new(test_rocket()).expect("valid rocket instance");

    let response = client.post("/users")
    .body(r#"{ "username": "TestUser", "password": "TestPassword", "email": "test.user@example.com" }"#)
    .header(ContentType::JSON).dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn create_user_duplicate() {
    let client = Client::new(rocket()).expect("valid rocket instance");

    let response1 = client.post("/users")
    .body(r#"{ "username": "TestUser", "password": "TestPassword", "email": "test.user@example.com" }"#)
    .header(ContentType::JSON).dispatch();

    let response2 = client.post("/users")
    .body(r#"{ "username": "TestUser", "password": "TestPassword", "email": "test.user@example.com" }"#)
    .header(ContentType::JSON).dispatch();

    assert_eq!(response1.status(), Status::Ok);
    assert_eq!(response2.status(), Status::Conflict);
}
