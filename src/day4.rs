use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Reindeer {
    name: String,
    strength: i64,
    speed: Option<f32>,
    height: Option<i64>,
    antler_width: Option<i64>,
    snow_magic_power: Option<i64>,
    favorite_food: Option<String>,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: Option<i64>,
}

#[derive(Serialize)]
pub struct ContestWinners {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[post("/4/strength", format = "json", data = "<reindeers>")]
pub fn strength(reindeers: Json<Vec<Reindeer>>) -> String {
    sum_strength(reindeers.into_inner()).to_string()
}

#[post("/4/contest", format = "json", data = "<reindeers>")]
pub fn contest(reindeers: Json<Vec<Reindeer>>) -> Json<ContestWinners> {
    let reindeers = reindeers.into_inner();
    let fastest = fastest_reindeer(&reindeers);
    let tallest = tallest_reindeer(&reindeers);
    let magician = magician_reindeer(&reindeers);
    let consumer = consumer_reindeer(&reindeers);

    let winners = ContestWinners {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name,
            tallest.antler_width.unwrap()
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name,
            magician.snow_magic_power.unwrap()
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name,
            consumer.favorite_food.unwrap()
        ),
    };

    Json(winners)
}

fn fastest_reindeer(reindeers: &Vec<Reindeer>) -> Reindeer {
    reindeers
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap()
        .clone()
}

fn tallest_reindeer(reindeers: &Vec<Reindeer>) -> Reindeer {
    reindeers
        .iter()
        .max_by(|a, b| a.height.partial_cmp(&b.height).unwrap())
        .unwrap()
        .clone()
}

fn magician_reindeer(reindeers: &Vec<Reindeer>) -> Reindeer {
    reindeers
        .iter()
        .max_by(|a, b| a.snow_magic_power.partial_cmp(&b.snow_magic_power).unwrap())
        .unwrap()
        .clone()
}

fn consumer_reindeer(reindeers: &Vec<Reindeer>) -> Reindeer {
    reindeers
        .iter()
        .max_by(|a, b| {
            a.candies_eaten_yesterday
                .partial_cmp(&b.candies_eaten_yesterday)
                .unwrap()
        })
        .unwrap()
        .clone()
}

fn sum_strength(reindeers: Vec<Reindeer>) -> i64 {
    reindeers.iter().map(|r| r.strength).sum::<i64>()
}

#[test]
fn task1_problem_example() {
    let reindeers: Vec<Reindeer> = serde_json::from_str(
        "
        [
            {
                \"name\": \"Dasher\",
                \"strength\": 5
            },
            {
                \"name\": \"Dancer\",
                \"strength\": 6
            },
            {
                \"name\": \"Prancer\",
                \"strength\": 4
            },
            {
                \"name\": \"Vixen\",
                \"strength\": 7
            }
        ]",
    )
    .unwrap();

    let result = sum_strength(reindeers);
    assert_eq!(result, 22);
}
