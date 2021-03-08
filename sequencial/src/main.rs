use std::time::Instant;

fn main() {
    let mut n: u128;
    n = 0;
    let now = Instant::now();
    loop {
        if is_prime(n) {
            println!("{}", n);
        }

        let then = Instant::now();
        if then.duration_since(now).as_secs() >= 1800 {
            break;
        }
        n += 1;
    }
    println!("THE END");
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}
