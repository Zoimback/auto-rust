use rust::{get_primes, is_prime};

fn main() {
    // Calcula y muestra todos los números primos hasta 1000.
    let primes = get_primes(1000);
    println!("Números primos hasta 1000:");
    for prime in &primes {
        println!("{}", prime);
    }

    // Verifica si un número específico es primo.
    let num = 997;
    if is_prime(num) {
        println!("El número {} es primo.", num);
    } else {
        println!("El número {} no es primo.", num);
    }
}
