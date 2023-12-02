use rocket::routes;

mod warmup;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![warmup::index, warmup::forced_500]);

    Ok(rocket.into())
}
