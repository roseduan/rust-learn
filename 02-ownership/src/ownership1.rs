fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let v = 50;

    // data 传递过去之后，所有权发生了转移
    if let Some(pos) = find_pos(data, v) {
        println!("find pos: {:?}", pos);
    } else {
        println!("did not find pos")
    }

    // 这里再次使用 data 变量会报错了
    // println!("data = {:?}", data);
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}
