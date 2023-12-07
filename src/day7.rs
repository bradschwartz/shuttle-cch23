use rocket::{get, http::CookieJar};

fn decode_base64(input: String) -> String {
    let decoded = simple_base64::decode(input).unwrap();
    String::from_utf8(decoded).unwrap()
}

#[get("/7/decode")]
pub fn decode(cookies: &CookieJar<'_>) -> String {
    let recipe_cookie = cookies.get("recipe").unwrap().value().to_string();
    decode_base64(recipe_cookie)
}

#[test]
fn task1_problem_example() {
    let recipe = "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==".to_string();
    let decoded = decode_base64(recipe);

    assert_eq!(decoded, r#"{"flour":100,"chocolate chips":20}"#);
}
