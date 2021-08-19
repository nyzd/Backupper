/**
 * Backupper Tests
 * */


#[cfg(test)]
mod tests {
    use crate::Rsa;
    #[test]
    fn rsa() {
        // Run Rsa test
        // Create a new RSA
        let rsa = Rsa::new();

        // Now encrypt some text with rsa
        let plain_text = "For the test".as_bytes();

        let encrypted_text = rsa.encrypt(&plain_text);

        // Decrypt text
        let decrypted_text = rsa.decrypt(&encrypted_text);

        // Check if decryption works
        assert_eq!(plain_text, decrypted_text);
    }
}
