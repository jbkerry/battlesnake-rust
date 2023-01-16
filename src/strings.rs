const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

pub fn run_test_code() {
    let data = "abc";
    let mut s = data.to_string();
    s.push_str("def");
    println!("Value is {s}");

    println!("Pig Latin = {}", pig_latin("How are you today"));

    let abc = '\u{CA0}';
    let def: u8 = 234;
    println!("{}_{abc}", def as char);
}

fn pig_latin(sentence: &str) -> String {
    let mut pl_sentence = String::new();
    for word in sentence.split_whitespace() {
        let first_letter = &word[0..1];
        if VOWELS.contains(&first_letter) {
            pl_sentence.push_str(&format!("{word}-hay "));
        } else {
            let rest_of_word = &word[1..];
            pl_sentence.push_str(&format!("{rest_of_word}-{first_letter}ay "))
        }
    }
    pl_sentence
}