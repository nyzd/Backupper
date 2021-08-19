mod cli;
use cli::*;
mod rsa;
pub use crate::rsa::*;
mod ftp;
use ftp::*;
use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    str,
};
mod test;
pub use test::*;

// TODO: handle Result
/// Get file buffer
fn file_to_bytes(file_path: &PathBuf) -> Vec<u8> {
    let o_file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(o_file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    buffer
}

/// Bytes to file
/// Write bytes to file
fn bytes_to_file(file_path: &PathBuf, bytes: &Vec<u8>) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(bytes)?;

    Ok(())
}

fn main() {
    // Get cli args
    let args = Cli::from_args();

    match args.subcmd {
        Subcommand::Encrypt(enc) => {
            // Start Encrypt file
            // Create a new Rsa struct for encrypt
            // with RSA algo
            let rsa = Rsa::new_with_files(&enc.pub_key_file, &enc.priv_key_file);

            // Read file to bytes
            let file = file_to_bytes(&enc.file);

            // Encrypt Bytes
            let encrypted_file = rsa.encrypt(&file);

            // Create encrypted file
            let _out_file = bytes_to_file(&enc.out, &encrypted_file);
        }
        Subcommand::Decrypt(dec) => {
            // Start Decrypt file
            // Create a new rsa struct
            let rsa = Rsa::new_with_files(&dec.pub_key_file, &dec.priv_key_file);

            // Read file to byte
            let file = file_to_bytes(&dec.enc_file);

            // Decrypt file
            let dec_file = rsa.decrypt(&file);

            // Create decrypted file
            let _out_file = bytes_to_file(&dec.out, &dec_file);
        }
        Subcommand::Upload(up) => {
            // Start upload file to ftp server
            let mut client = FtpClient::new(&up.server_addr, &up.username, &up.password);
            if up.encrypt {
                // First encrypt file and upload to ftp server
                // Create a new RSA struct
                let rsa = Rsa::new_with_files(&up.pub_key_file, &up.priv_key_file);

                // File to byte
                let file_bytes = file_to_bytes(&up.file);

                // Encrypt file
                let enc_file = rsa.encrypt(&file_bytes);

                // Create file
                let file = FtpFile {
                    content: &enc_file,
                    name: &up.file.to_str().unwrap().to_string(),
                };

                // Upload
                client.upload(&file);
            } else {
                // Dont encrypt file
                // File to bytes
                let file_bytes = file_to_bytes(&up.file);

                // Create File
                let file = FtpFile {
                    content: &file_bytes,
                    name: &up.file.to_str().unwrap().to_string(),
                };

                // Upload
                client.upload(&file);
            }
        }
        Subcommand::Download(dwn) => {
            if dwn.encrypted {
                // Start Download file
                // Create a Ftp Client
                let mut ftp = FtpClient::new(&dwn.server_addr, &dwn.username, &dwn.password);
    
                // Now Download file
                let file = ftp.download(&dwn.file);

                // Decrypt file
                // Start create a RSA Struct
                let rsa = Rsa::new_with_files(&dwn.pub_key_file, &dwn.priv_key_file);

                // Start decrypt file
                let decrypted = rsa.decrypt(&file);

                // Save decrypted file
                let _saved_file = bytes_to_file(&dwn.out, &decrypted);
            }
            else {
                // Start Download file
                // Create a Ftp Client
                let mut ftp = FtpClient::new(&dwn.server_addr, &dwn.username, &dwn.password);

                // Now Download file
                let file = ftp.download(&dwn.file);

                // Save file
                let _saved_file = bytes_to_file(&dwn.out, &file);

            }

        }
    }
}
