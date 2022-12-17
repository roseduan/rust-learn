use std::mem::{align_of, size_of};

// #[repr(C)] // 这个 trait 的意思是和 C 语言保持一致，不进行自动内存对齐
struct s1 {
    a: u8,
    b: u16,
    c: u8,
}

struct s2 {
    a: u8,
    c: u8,
    b: u16,
}

fn main() {
    println!("size of s1: {}, s2: {}", size_of::<s1>(), size_of::<s2>());
    println!("align of s1: {}, s2: {}", align_of::<s1>(), align_of::<s2>());
}
