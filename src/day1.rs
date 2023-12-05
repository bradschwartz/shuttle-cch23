use rocket::get;

#[get("/1/<num1>/<num2>")]
pub fn cube_bits(num1: i64, num2: i64) -> String {
    let xor = num1 ^ num2;
    let powed = xor.pow(3);
    print!("{}", powed);
    powed.to_string()
}

#[test]
fn problem_example() {
    let result = cube_bits(4, 8);
    assert_eq!(result, "1728")
}
