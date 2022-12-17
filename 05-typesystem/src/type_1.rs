use std::collections::BTreeMap;

fn main() {
    // 可以在一个作用域中，通过变量的上下文自动推导
    let mut tm = BTreeMap::new();

    tm.insert("key1", "value1");
    tm.insert("key2", "value2");

    println!("map: {:?}", tm);

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 这里无法自动推导，必须手动声明
    let even_numbers: Vec<_> = numbers.clone().into_iter().filter(|n| n % 2 == 0).collect();

    println!("even_numbers: {:?}", even_numbers);

    // 或者也可以让 collect 方法返回一个明确的值
    let even_numbers1 = numbers
        .clone()
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect::<Vec<_>>();

    println!("even_numbers1: {:?}", even_numbers1);
}
