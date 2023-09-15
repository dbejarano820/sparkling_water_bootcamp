use num_bigint_dig::{BigUint, ToBigUint};
use num_traits::ToPrimitive;

#[derive(Debug, Clone)]
pub struct PublicKey {
    phi: BigUint,
    n: BigUint,
}

#[derive(Debug, Clone)]
pub struct PrivateKey {
    d: BigUint,
    n: BigUint,
}

impl PublicKey {
    // Public constructor method
    pub fn new(phi: BigUint, n: BigUint) -> Self {
        PublicKey { phi, n }
    }

    pub fn encrypt(&self, msg: String) -> Vec<BigUint> {
        let msg_bytes = msg.as_bytes();
        msg_bytes
            .iter()
            .map(|&byte| {
                let byte_biguint = byte.to_biguint().unwrap();
                byte_biguint.modpow(&self.phi, &self.n)
            })
            .collect()
    }

    // Getter for `phi`
    pub fn get_phi(&self) -> &BigUint {
        &self.phi
    }

    // Getter for `n`
    pub fn get_n(&self) -> &BigUint {
        &self.n
    }
}

impl PrivateKey {
    // Public constructor method
    pub fn new(d: BigUint, n: BigUint) -> Self {
        PrivateKey { d, n }
    }

    pub fn decrypt(&self, ciphertext: Vec<BigUint>) -> String {
        let decrypted_bytes: Vec<u8> = ciphertext
            .iter()
            .map(|c| c.modpow(&self.d, &self.n).to_u8().unwrap())
            .collect();
        String::from_utf8(decrypted_bytes).expect("Decryption failed")
    }

    // Getter for `phi`
    pub fn get_d(&self) -> &BigUint {
        &self.d
    }

    // Getter for `n`
    pub fn get_n(&self) -> &BigUint {
        &self.n
    }
}
