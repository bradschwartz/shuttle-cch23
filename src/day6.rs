use rocket::{post, serde::json::Json};

#[derive(serde::Serialize)]
pub struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elfs_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf: usize,
}

#[post("/6", data = "<text>")]
pub fn count_elf(text: String) -> Json<ElfCount> {
    let elfs = text.matches("elf").count();
    let elfs_on_a_shelf = text.matches("elf on a shelf").count();

    // we calculate `shelf` _not_ prefixed by `elf on a` to avoid double counting
    // just remove the entire string `elf on a shelf` and count the remaining `shelf`
    let shelf = text.replace("elf on a shelf", "").matches("shelf").count();

    Json(ElfCount {
        elf: elfs,
        elfs_on_a_shelf: elfs_on_a_shelf,
        shelf: shelf,
    })
}

#[test]
fn task1_problem_example() {
    let text = "The mischievous elf peeked out from behind the toy workshop,
    and another elf joined in the festive dance.
    Look, there is also an elf on that shelf!";
    let result = count_elf(text.to_string());

    assert_eq!(result.elf, 4);
    assert_eq!(result.elfs_on_a_shelf, 0);
    assert_eq!(result.shelf, 1);
}

#[test]
fn task2_problem_example() {
    let text = "there is an elf on a shelf on an elf.
    there is also another shelf in Belfast.";
    let result = count_elf(text.to_string());

    assert_eq!(result.elf, 5);
    assert_eq!(result.elfs_on_a_shelf, 1);
    assert_eq!(result.shelf, 1)
}
