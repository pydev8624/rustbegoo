fn main() {
    let mut name = String::from("rust is awesome");
    name = name.replace("rust", "python");
    print!("{}\n", name);
}
