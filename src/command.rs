use super::{Cli, FtpClient, FtpFile, Rsa, Subcommand};
use crate::utils::*;
use structopt::StructOpt;
use std::{path::Path, time::Duration};
use notify::{RecursiveMode};
use notify_debouncer_mini::new_debouncer;

/// Command
///
/// Run function will start Cli
pub trait Command {
    fn run() -> ();
}

impl Command for Cli {
    fn run() -> () {
        // Get cli args
        let args = Self::from_args();

        match &args.subcmd {
            Subcommand::Encrypt(enc) => {
                // Start Encrypt file
                // Create a new Rsa struct for encrypt
                // with RSA algo
                let rsa = Rsa::new_with_files(&enc.pub_key_file, &enc.priv_key_file);

                // Read file to bytes
                let file = file_to_bytes(&enc.file).unwrap();

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
                let file = file_to_bytes(&dec.enc_file).unwrap();

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
                    let file_bytes = file_to_bytes(&up.file).unwrap();

                    // Encrypt file
                    let enc_file = rsa.encrypt(&file_bytes);

                    // Calculate hash of file
                    let hash = sha256_hash(&enc_file);

                    // Print out hash for user
                    println!("Encrypted File HASH : {:?}", hex::encode(hash));

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
                    let file_bytes = file_to_bytes(&up.file).unwrap();

                    // Calculate hash of file
                    let hash = sha256_hash(&file_bytes);

                    // Print out hash for user
                    println!("File HASH : {:?}", hex::encode(hash));

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

                    // Calculate hash of file
                    let hash = sha256_hash(&file);

                    // Print out hash for user
                    println!("Encrypted File HASH : {:?}", hex::encode(hash));

                    // Decrypt file
                    // Start create a RSA Struct
                    let rsa = Rsa::new_with_files(&dwn.pub_key_file, &dwn.priv_key_file);

                    // Start decrypt file
                    let decrypted = rsa.decrypt(&file);

                    // Save decrypted file
                    let _saved_file = bytes_to_file(&dwn.out, &decrypted);
                } else {
                    // Start Download file
                    // Create a Ftp Client
                    let mut ftp = FtpClient::new(&dwn.server_addr, &dwn.username, &dwn.password);

                    // Now Download file
                    let file = ftp.download(&dwn.file);

                    // Calculate hash of file
                    let hash = sha256_hash(&file);

                    // Print out hash for user
                    println!("File HASH : {:?}", hex::encode(hash));

                    // Save file
                    let _saved_file = bytes_to_file(&dwn.out, &file);
                }
            },

            // Watch file
            // If changed then upload it to the server
            Subcommand::Watch(w) => {
                let (tx, rx) = std::sync::mpsc::channel();

                let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx).unwrap();

                debouncer
                    .watcher()
                    .watch(Path::new(&w.dir), RecursiveMode::Recursive)
                    .unwrap();

                for res in rx {
                    match res {
                        Ok(event) => {
                            // Start upload file to ftp server
                            let mut client = FtpClient::new(&w.server_addr, &w.username, &w.password);
                            let event = &event[0];
                            if w.encrypt {
                                // First encrypt file and upload to ftp server
                                // Create a new RSA struct
                                let rsa = Rsa::new_with_files(&w.pub_key_file, &w.priv_key_file);

                                // File to byte
                                let Ok(file_bytes) = file_to_bytes(&event.path) else {
                                    println!("Error when reading a file {} ", event.path.display());
                                    continue;
                                };

                                // Encrypt file
                                let enc_file = rsa.encrypt(&file_bytes);

                                // Calculate hash of file
                                let hash = sha256_hash(&enc_file);

                                // Print out hash for user
                                println!("Encrypted File HASH : {:?}", hex::encode(hash));


                                // find the file name to upload with that name to the server
                                let file_name = Path::new(&event.path).file_name().unwrap().to_str().unwrap();

                                // Create file
                                let file = FtpFile {
                                    content: &enc_file,
                                    name: &file_name.to_string(),
                                };

                                // Upload
                                client.upload(&file);

                            } else {
                                // Dont encrypt file
                                // File to bytes
                                let Ok(file_bytes) = file_to_bytes(&event.path) else {
                                    println!("Error when reading a file {} ", event.path.display());
                                    continue;
                                };

                                let hash = sha256_hash(&file_bytes);

                                // Print out hash for user
                                println!("File HASH : {:?}", hex::encode(hash));

                                // find the file name to upload with that name to the server
                                let file_name = Path::new(&event.path).file_name().unwrap().to_str().unwrap();

                                // Create File
                                let file = FtpFile {
                                    content: &file_bytes,
                                    name: &file_name.to_string(),
                                };

                                // Upload
                                client.upload(&file);
                            }
                        },
                        
                        Err(e) => println!("watch error: {:?}", e),
                    }
                }
            }
        }
    }
}
