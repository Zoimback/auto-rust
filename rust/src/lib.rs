/// Comprueba si un número es primo.
/// Un número primo es aquel mayor que 1 y que solo es divisible por 1 y por sí mismo.
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let limite = (n as f64).sqrt() as u32;
    for i in 2..=limite {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Devuelve un vector con todos los números primos desde 2 hasta el límite dado (inclusive).
pub fn get_primes(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    for i in 2..=limit {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes
}
