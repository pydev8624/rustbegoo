fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let text = "Rust,is,awesome";
    let iter = text.split(',');
    print_type_of(&iter);
}
