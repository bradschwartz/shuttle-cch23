use rocket::{post, serde::json::Json};

#[derive(serde::Serialize)]
pub struct ElfCount {
    elf: usize,
}

#[post("/6", data = "<text>")]
pub fn count_elf(text: String) -> Json<ElfCount> {
    let elfs = text.matches("elf").count();
    Json(ElfCount { elf: elfs })
}

#[test]
fn task1_problem_example() {
    let text = "The mischievous elf peeked out from behind the toy workshop,
    and another elf joined in the festive dance.
    Look, there is also an elf on that shelf!";
    let result = count_elf(text.to_string());

    assert_eq!(result.elf, 4)
}
