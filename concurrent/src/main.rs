use num_cpus;
use std::time::Instant;
use threadpool::ThreadPool;

fn main() {
    let mut n: usize;
    n = 0;
    //Criando um pool de threads com a quantidade de processadores logicos no PC.
    //Nesse caso 4.
    let pool = ThreadPool::new(num_cpus::get());
    let now = Instant::now();
    loop {
        if n <= 10_000_000 {
            pool.execute(move || task(n));
            n += 1;
        }

        let then = Instant::now();
        if then.duration_since(now).as_secs() >= 1800 {
            break;
        }
    }
    println!("THE END");
}
fn task(n: usize) {
    if is_prime(n) {
        println!("{}", n);
    }
}
fn is_prime(n: usize) -> bool {
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