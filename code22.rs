fn main() {
    let data = String::from("ali,reza,mina");
    for name in data.split(',') {
        println!("{}", name);
    }
}
