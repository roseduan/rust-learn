// 判断哪些数据结构实现了 Copy trait

fn is_copy<T: Copy>() {}

fn type_impl_copy_trait() {
    is_copy::<bool>();
}

fn main() {
    type_impl_copy_trait();
}
