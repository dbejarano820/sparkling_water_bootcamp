#[cfg(test)]
pub mod utils_test {
    use crate::utils::{compute_totient, gcd, generate_prime, mod_inv};
    use num_bigint_dig::prime::probably_prime;
    use num_bigint_dig::BigUint;

    #[test]
    fn test_generate_prime() {
        let bits = 256;
        let prime = generate_prime(bits);
        // Implement your own prime check or use a library
        assert!(probably_prime(&prime, 4));
    }

    #[test]
    fn test_compute_totient() {
        let p = BigUint::from(11_u64);
        let q = BigUint::from(17_u64);
        let totient = compute_totient(p.clone(), q.clone());
        assert_eq!(
            totient,
            (p - BigUint::from(1_u64)) * (q - BigUint::from(1_u64))
        );
    }

    #[test]
    fn test_gcd() {
        let a = BigUint::from(12_u64);
        let b = BigUint::from(15_u64);
        let result = gcd(a.clone(), b.clone());
        assert_eq!(result, BigUint::from(3_u64));
    }

    #[test]
    fn test_mod_inv() {
        let a = BigUint::from(3_u64);
        let m = BigUint::from(11_u64);
        let inv = mod_inv(&a, &m).expect("Modular inverse does not exist");
        assert_eq!(inv, BigUint::from(4_u64));
    }
}

#[cfg(test)]
pub mod rsa_test {
    use crate::keys::{PrivateKey, PublicKey};
    use crate::utils::{compute_totient, generate_prime, mod_inv};
    use num_bigint_dig::BigUint;

    #[test]
    fn test_rsa_encryption_decryption() {
        let bits = 256; // Using smaller size for the test to run faster
        let e = 65537_u64;
        let p = generate_prime(bits);
        let q = generate_prime(bits);
        let n = &p * &q;
        let totient = compute_totient(p.clone(), q.clone());

        let pub_key = PublicKey::new(BigUint::from(e), n.clone());
        let d = mod_inv(pub_key.get_phi(), &totient).expect("Modular inverse does not exist");
        let priv_key = PrivateKey::new(d, n.clone());

        let original_message = "Hello world!".to_string();
        let encrypted_message = pub_key.encrypt(original_message.clone());
        let decrypted_message = priv_key.decrypt(encrypted_message.clone());

        assert_eq!(original_message, decrypted_message);
    }
}
