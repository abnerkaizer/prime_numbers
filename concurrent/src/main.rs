use num_cpus;
use std::time::Instant;
use threadpool::ThreadPool;
/// Programa concorrente que verifica em uma sequência de números
/// por 30 min quais entre eles são primos ou não.
fn main() {
    //O tipo usize é sem sinal e depende da arquitetura
    //que está sendo compilado se for 32 bits compila
    //para u32 e se for 64 bits, meu caso,para u64.
    let mut n: usize;
    n = 0;
    //Criando um pool de threads com a quantidade de processadores logicos no PC.
    //Nesse caso 4.
    let pool = ThreadPool::new(num_cpus::get());
    //Marca o inicio do processamento.
    let now = Instant::now();
    //Tipo de laço em Rust equivale a um "while true {}".
    loop {
        //Coloquei um limite menor que o maximo possivel
        //porque estava estourando a memoria RAM e a swap.
        //Ocorre devido a fila de tarefas do pool.
        if n <= 10_000_000 {
            //Manda o pool executar a tarefa passando uma clojure
            //com a função a ser executada.
            pool.execute(move || task(n));
            n += 1;
        }
        //Se chega a 30 min(1800s) quebra o loop.
        let then = Instant::now();
        if then.duration_since(now).as_secs() >= 1800 {
            break;
        }
    }
    //Aqui o pool sai de escopo então ele será liberado da memória junto do programa.
    //Isso chama lifetime em Rust, garantindo que uma variável viva enquanto o escopo
    //que ela está existir.
}
/// Função a ser executada pelas threads do pool.
fn task(n: usize) {
    if is_prime(n) {
        println!("{}", n);
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
