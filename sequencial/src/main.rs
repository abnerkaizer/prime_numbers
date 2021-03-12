use std::time::Instant;

/// Programa sequencial que verifica em uma lista de números
/// quais entre eles são primos ou não.
fn main() {
    //O tipo usize é sem sinal e depende da arquitetura
    //que está sendo compilado se for 32 bits compila
    //para u32 e se for 64 bits, meu caso,para u64.
    let mut n: usize = 0;

    let now = Instant::now();

    loop {
        if is_prime(n) {
            println!("{}", n);
        }
        //Limite devido ao problema da solução
        //concorrente que estoura memoria
        if n == 100_000_000 {
            let duration = Instant::now().duration_since(now).as_secs();
            println!("Duration: {}s", duration);
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
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
