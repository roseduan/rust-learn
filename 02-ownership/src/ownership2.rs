fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let data1 = data; // 所有权转移到了 data1

    println!("sum of data1: {}", sum(data1));
    // println!("data1: {:?}", data1) // 报错，data1的所有权也被转移了
    // println!("data: {:?}", data) // 报错
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}
