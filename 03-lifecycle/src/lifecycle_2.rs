fn main() {
    let s1 = String::from("Roseduan");
    let s2 = String::from("Jack Zhang");

    let result = max(&s1, &s2);

    println!("the bigger one is {}", result);
}

// 这里无法确定生命周期
// fn max(s1: &str, s2: &str) ->&str {
//     if s1 > s2 {
//         s1
//     } else {
//         s2
//     }
// }

// 添加了生命周期标识符之后，就能编译通过了
fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
