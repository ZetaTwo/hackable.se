use super::rocket;
use crate::models::password_hash::PasswordHashingConfig;
use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use rocket::Rocket;

#[database("test")]
pub struct TestDbConn(diesel::SqliteConnection);

embed_migrations!();

fn run_test_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = TestDbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn test_rocket() -> rocket::Rocket {
    super::rocket_base()
        .attach(TestDbConn::fairing())
        .attach(AdHoc::on_attach(
            "Test Database Migrations",
            run_test_db_migrations,
        ))
        .attach(AdHoc::on_attach("Argon2 secret key", |rocket| match rocket
            .config()
            .get_string("argon_secret_key")
        {
            Err(err) => {
                error!("Failed to read Argon2 secret key from config");
                Err(rocket)
            }
            Ok(argon_secret_key) => Ok(rocket.manage(PasswordHashingConfig::new(argon_secret_key))),
        }))
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
