fn toprime() {
    for num in 2..100 {
        let mut is_prime = true;
        for m in 2..num {
            if num % m == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            println!("{} is prime", num);
        }
    }
}

fn main() {
    toprime();
}
