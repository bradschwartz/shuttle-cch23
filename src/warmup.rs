use rocket::{get, http::Status};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
pub fn forced_500() -> Status {
    Status::InternalServerError
}
