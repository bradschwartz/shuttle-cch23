use std::path::PathBuf;

use rocket::get;

#[get("/1/<num_parts..>")]
pub fn cube_bits(num_parts: PathBuf) -> String {
    let mut xor: i64 = 0;
    num_parts.iter().for_each(|part| {
        xor ^= part.to_str().unwrap().parse::<i64>().unwrap();
    });
    let powed = xor.pow(3);
    print!("{}", powed);
    powed.to_string()
}

#[test]
fn task1_problem_example() {
    let num_parts = PathBuf::from("4/8");
    let result = cube_bits(num_parts);
    assert_eq!(result, "1728")
}

#[test]
fn task2_problem_example() {
    let num_parts = PathBuf::from("10");
    let result = cube_bits(num_parts);
    assert_eq!(result, "1000");

    let num_parts = PathBuf::from("4/5/8/10");
    let result = cube_bits(num_parts);
    assert_eq!(result, "27");
}
