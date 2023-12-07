use rocket::routes;

mod day1;
mod day4;
mod day6;
mod day7;
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
            day6::count_elf,
            day7::decode,
        ],
    );

    Ok(rocket.into())
}
