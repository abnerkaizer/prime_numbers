use std::time::Instant;

/// Programa sequencial que verifica em uma lista de números
/// por 30 min quais entre eles são primos ou não.
fn main() {
    //O tipo usize é sem sinal e depende da arquitetura
    //que está sendo compilado se for 32 bits compila
    //para u32 e se for 64 bits, meu caso,para u64.
    let mut n: usize;
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
}
/// Função que verifica se um número é primo.
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
