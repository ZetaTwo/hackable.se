use super::rocket;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

/*#[test]
fn hello_world() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}*/

#[test]
fn create_user() {
    let client = Client::new(rocket()).expect("valid rocket instance");

    let req = client.post("/users")
    .body(r#"{ "username": "TestUser", "password": "TestPassword", "email": "test.user@example.com" }"#)
    .header(ContentType::JSON);

    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
}
