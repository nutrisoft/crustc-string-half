fn main() {
    let word = "The dream my father told me";

    let mid = word.char_indices().count() / 2;
    let (first_half, second_half) = word.split_at(mid);
    assert_eq!(first_half, "The dream my ");
    assert_eq!(second_half, "father told me");

    let (first_part, second_part) = word.split_at(9);
    assert_eq!(first_part, "The dream");
    assert_eq!(second_part, " my father told me");

    if word.is_char_boundary(9) {
        let (first_part, second_part) = word.split_at(9);
        println!("First part: {}", first_part);
        println!("Second part: {}", second_part);
    } else {
        println!("Index {} is not a character boundary.", 9);
    }
}
