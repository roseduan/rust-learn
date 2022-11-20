fn main() {
    let s1 = "Hello World";

    let res = first(s1);
    println!("first of s1: {}", res);
}

fn first(s: &str) ->&str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos]
    }
}
