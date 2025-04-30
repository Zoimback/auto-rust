mod test_helpers;

use rust::{get_primes, is_prime};

#[test]
fn integration_test_is_prime() {
    // Llamada a la función auxiliar de configuración.
    test_helpers::setup_env();
    assert!(is_prime(7));
    assert!(!is_prime(10));
}

#[test]
fn integration_test_get_primes() {
    test_helpers::setup_env();
    let expected = vec![2, 3, 5, 7];
    assert_eq!(get_primes(10), expected);
}
