use rand::rngs::OsRng;
use rsa::pkcs8::{FromPrivateKey, FromPublicKey, ToPrivateKey, ToPublicKey};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use std::path::PathBuf;

const RNG: OsRng = OsRng;

/// Rsa Struct
pub struct Rsa {
    /// Public Key
    pub_key: RsaPublicKey,

    /// Private Key
    priv_key: RsaPrivateKey,
}

// This impl uses files
impl Rsa {
    /// Create a new public and private key
    pub fn new() -> Self {
        let bits = 2048;
        let mut rng = RNG;

        // Create Private key First
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a private key");

        // And create public key
        let public_key = RsaPublicKey::from(&private_key);

        // Return private and public key
        Self {
            pub_key: public_key,
            priv_key: private_key,
        }
    }

    /// Create a new rsa struct with pub and priv key files
    pub fn new_with_files(pub_key_file: &PathBuf, priv_key_file: &PathBuf) -> Self {
        // If public key and private key file exists read files
        if pub_key_file.exists() && priv_key_file.exists() {
            let rsa = Self::from_files(pub_key_file, priv_key_file);
            Self {
                pub_key: rsa.pub_key,
                priv_key: rsa.priv_key,
            }
        } else {
            // Else create a files
            let rsa = Self::to_files(pub_key_file, priv_key_file);
            Self {
                pub_key: rsa.pub_key,
                priv_key: rsa.priv_key,
            }
        }
    }

    /// Create new public and private keys and save to file
    fn to_files(pub_key_file: &PathBuf, priv_key_file: &PathBuf) -> Self {

        // Create RSA public and private keys
        let rsa = Self::new();

        // Right public and private keys to files
        let _pub_key_file = rsa.pub_key.write_public_key_pem_file(pub_key_file);
        let _priv_key_file = rsa.priv_key.write_pkcs8_pem_file(priv_key_file);

        Self {
            pub_key: rsa.pub_key,
            priv_key: rsa.priv_key,
        }
    }

    /// Get public and private keys from files
    fn from_files(pub_key_file: &PathBuf, priv_key_file: &PathBuf) -> Self {
        // First find public key
        let pub_key = FromPublicKey::read_public_key_pem_file(pub_key_file).unwrap();
        let priv_key = FromPrivateKey::read_pkcs8_pem_file(priv_key_file).unwrap();

        Self {
            pub_key: pub_key,
            priv_key: priv_key,
        }
    }

    /// Encrypt
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let mut rng = RNG;

        let encrypted = self.pub_key.encrypt(&mut rng, padding, data).unwrap();

        encrypted
    }

    /// Decrypt
    ///
    /// enc_data -> Encrypted data (CypherText)
    pub fn decrypt(&self, enc_data: &Vec<u8>) -> Vec<u8> {
        let padding = PaddingScheme::new_pkcs1v15_encrypt();

        let dec_data = self
            .priv_key
            .decrypt(padding, &enc_data)
            .expect("failed to decrypt");

        dec_data
    }
}
