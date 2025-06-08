fn main() {
    let mut count = 0;
    loop {
        println!("count: {}", count);
        count += 1;
        if count == 5 {
            break;
        }
    }
}
