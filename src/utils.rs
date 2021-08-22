use sha2::{Digest, Sha256};
use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

// TODO: handle Result
/// Get file buffer
pub fn file_to_bytes(file_path: &PathBuf) -> Vec<u8> {
    let o_file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(o_file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    buffer
}

/// Bytes to file
/// Write bytes to file
pub fn bytes_to_file(file_path: &PathBuf, bytes: &Vec<u8>) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(bytes)?;

    Ok(())
}

/// HASH
type Hash = sha2::digest::generic_array::GenericArray<
    u8,
    sha2::digest::generic_array::typenum::UInt<
        sha2::digest::generic_array::typenum::UInt<
            sha2::digest::generic_array::typenum::UInt<
                sha2::digest::generic_array::typenum::UInt<
                    sha2::digest::generic_array::typenum::UInt<
                        sha2::digest::generic_array::typenum::UInt<
                            sha2::digest::generic_array::typenum::UTerm,
                            sha2::digest::consts::B1,
                        >,
                        sha2::digest::consts::B0,
                    >,
                    sha2::digest::consts::B0,
                >,
                sha2::digest::consts::B0,
            >,
            sha2::digest::consts::B0,
        >,
        sha2::digest::consts::B0,
    >,
>;

/// SHA 256 calculate
pub fn sha256_hash(bytes: &[u8]) -> Hash {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(bytes);

    // read hash digest and consume hasher
    let result = hasher.finalize();

    result
}
