pub fn split_string(s: String, delimeter: char, field: usize) -> String {
    let mut result: String = String::new();
    let mut field_count: usize = 0;
    for c in s.chars() {
        if c == delimeter {
            field_count += 1;
            if field_count == field {
                break;
            }
        } else if field_count == field - 1 {
            result.push(c);
        }
    }
    result
}
