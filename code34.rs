fn main() {
    let result = loop {
        let x = 3;
        if x == 3 {
            break x * 2;
        }
    };
    println!("Result: {}", result);
}
