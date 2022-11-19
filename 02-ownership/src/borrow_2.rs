// 借用的生命周期

// 情况 1
// fn main() {
//     // let r = local_ref();
//     // println!("r: {:p}", r)
// }

// 变量 a 的生命周期更短，无法在 main 中引用
// fn local_ref<'a>() -> &'a i32 {
//     let a = 32;
//     &a
// }


// 情况 2
// data 和 v 的生命周期一样，所以这里不会报错
// fn main() {
//     let mut data = Vec::new();
//     let v = 34;
//     data.push(v);

//     println!("data: {:?}", data);
// }

// 情况 3
// data引用了生命周期更短的指针 v，所以会报错
// fn main() {
//     let mut data = Vec::new();
//     push_local_ref(&mut data);
//     println!("data: {:?}", data);
// }

// fn push_local_ref(data: &mut Vec<&u32>) {
//     let v = 100;
//     data.push(&v); // borrowed value does not live long enough
// }

fn main() {
    println!("Hello World");
}
