use rocket::routes;

mod day1;
mod day4;
mod warmup;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount(
        "/",
        routes![
            warmup::index,
            warmup::forced_500,
            day1::cube_bits,
            day4::strength,
            day4::contest,
        ],
    );

    Ok(rocket.into())
}
