pub fn String_excercise(s: &str) {
    println!("String excercise: {}", s); // we cant change this value
}

pub fn string_length(s: &str) -> usize {
    s.len()
}
pub fn string_concatenation(s1: &str, s2: &str) -> String {
    let mut result = String::from(s1);
    result.push_str(s2);
    result
}
pub fn string_repetition(s: &str, n: usize) -> String {
    s.repeat(n)
}
pub fn print_string(s: String) {
    println!("{}", s);
}
pub fn string_vector() {
    let sentence = "Hello, world!".to_string();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);
}
