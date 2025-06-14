fn info() -> (String, i32) {
    let name = String::from("Ali");
    let age = 30;
    (name, age)
}

fn main() {
    let (name, age) = info();
    println!("{} is {} years old", name, age);
}
