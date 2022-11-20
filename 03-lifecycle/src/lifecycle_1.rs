fn main() {
    demo2();
}

fn demo1() {
    // let x;
    {
        let y = 100;

        // y 的生命周期没有 x 长
        // 所有这里不能引用x
        // x = &y;
    }

    // println!("x = {:?}", x);
}

fn demo2() {
    let y = 42;

    // x 和 y 具有相同的生命周期，所以可以直接引用 x
    let x = &y;

    println!("x = {:?}", x);
}
