use num_bigint_dig::BigUint;
use rsa::keys::{PrivateKey, PublicKey};
use rsa::utils::{compute_totient, gcd, generate_prime, mod_inv};

fn main() {
    const BITS: usize = 1024;
    const E: u64 = 65537_u64;

    let p = generate_prime(BITS);
    let q = generate_prime(BITS);

    println!("Generated primes: \np: {}\nq: {}", p, q);

    let n = &p * &q;
    println!("n: {}", n);

    let totient = compute_totient(p, q);
    println!("totient: {}", totient);

    let pub_key = PublicKey::new(BigUint::from(E), n.clone());

    let test = gcd(pub_key.get_phi().clone(), totient.clone());
    println!("must be 1 -> gcd: {}", test);

    let d = mod_inv(pub_key.get_phi(), &totient).expect("Modular inverse does not exist");
    println!("The modular multiplicative inverse is {}", d);

    let priv_key = PrivateKey::new(d, n.clone());

    let test = priv_key.get_d().clone() * pub_key.get_phi().clone();
    println!("must be 1 -> d * phi: {}", test.clone() % totient.clone());

    let original_message = "Hello world!".to_string();
    let encrypted_message = pub_key.encrypt(original_message.clone());
    let decrypted_message = priv_key.decrypt(encrypted_message.clone());
    println!("Original message: {}", original_message);
    println!("Encrypted message: {:?}", encrypted_message);
    println!("Decrypted message: {}", decrypted_message);
}
