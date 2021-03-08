use std::time::Instant;
use threadpool::ThreadPool;
use num_cpus;

fn main() {
	let mut n:u128;
	n = 0;
	//Criando um pool de threads com a quantidade de processadores logicos no PC.
	//Nesse caso 4.
	let pool = ThreadPool::new(num_cpus::get());
	let now = Instant::now();
    loop {

    	pool.execute(move || task(n));
    	
    	let then = Instant::now();
    	if then.duration_since(now).as_secs()>=1800 || n==u128::MAX {
    		break;
    	}
    	n+=1;
    }
    println!("THE END");
}
fn task(n:u128){
	if is_prime(n) {
		println!("{}", n);
	}
}
fn is_prime(n:u128) ->bool{
	if n<=1 {
		return false;
	}

	for a in 2..n {
		if n%a == 0 {
			return false;
		}
	}
	true
}