fn main() {
    let s = "I am roseduan";
    let a = s.find("aro");

    if a.is_some() {
        let res = a.unwrap();
        println!("res = {}", res);
    }

    let ss = "hello world".to_owned();

    let mut s1 = ss.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello = {}, s1 = {}, ss = {}", hello, s1, ss);
}

pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];

        let suffix = &s[i + delimiter.len_utf8()..];
        *s = suffix;
        prefix
    } else {
        // 如果没找到，返回整个字符串
        let prefix = *s;
        *s = "";
        prefix
    }
}
