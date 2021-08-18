pub use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum Subcommand {
    Encrypt(EncryptCmd),
    Decrypt(DecryptCmd),
    Upload(UploadCmd),
    Download(DownloadCmd),
}

// RSA sub command
#[derive(Debug, PartialEq, StructOpt)]
pub struct EncryptCmd {
    #[structopt(
        name = "File to encrypt",
        parse(from_os_str),
        short = "f",
        long = "file",
        required(true)
    )]
    pub file: std::path::PathBuf,
    #[structopt(
        name = "Public key pem file",
        parse(from_os_str),
        long = "public",
        required(true),
        help = "If you dont have , just pass name i will create it"
    )]
    pub pub_key_file: std::path::PathBuf,
    #[structopt(
        name = "Private key pem file",
        parse(from_os_str),
        long = "private",
        required(true),
        help = "If you dont have , just pass name i will create it"
    )]
    pub priv_key_file: std::path::PathBuf,
    #[structopt(
        parse(from_os_str),
        short = "o",
        long = "output",
        required(true),
        help = "Final file path. example -> ./out"
    )]
    pub out: std::path::PathBuf,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct DecryptCmd {
    #[structopt(
        name = "Encrypted file",
        parse(from_os_str),
        short = "f",
        long = "file",
        required(true)
    )]
    pub enc_file: std::path::PathBuf,
    #[structopt(parse(from_os_str), long = "public", required(true))]
    pub pub_key_file: std::path::PathBuf,
    #[structopt(parse(from_os_str), long = "private", required(true))]
    pub priv_key_file: std::path::PathBuf,
    #[structopt(
        name = "Out file path(name)",
        parse(from_os_str),
        short = "o",
        long = "output",
        required(true),
        help = "This needs file extention you encrypted. example -> if you encrypted .png file you need decrypt it with .png extention -> ./decrypted.png"
    )]
    pub out: std::path::PathBuf,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct UploadCmd {
    #[structopt(
        name = "File to opload",
        parse(from_os_str),
        short = "f",
        long = "file",
        required(true)
    )]
    pub file: std::path::PathBuf,

    #[structopt(name = "Server address", short = "s", long = "server", required(true))]
    pub server_addr: String,

    #[structopt(name = "Username", short = "u", long = "username", required(true))]
    pub username: String,

    #[structopt(name = "Password", short = "p", long = "password", required(true))]
    pub password: String,

    #[structopt(name = "Encrypt file", short = "e", long = "encrypt")]
    pub encrypt: bool,

    #[structopt(parse(from_os_str), long = "public", required_if("encrypt", "true"))]
    pub pub_key_file: std::path::PathBuf,

    #[structopt(parse(from_os_str), long = "private", required_if("encrypt", "true"))]
    pub priv_key_file: std::path::PathBuf,
}


#[derive(Debug, PartialEq, StructOpt)]
pub struct DownloadCmd {
    #[structopt(
        name = "FileName to Download",
        short = "f",
        long = "file",
        required(true)
    )]
    pub file: String,

    #[structopt(name = "Server address", short = "s", long = "server", required(true))]
    pub server_addr: String,

    #[structopt(name = "Username", short = "u", long = "username", required(true))]
    pub username: String,

    #[structopt(name = "Password", short = "p", long = "password", required(true))]
    pub password: String,

    #[structopt(
        name = "Out file path(name)",
        parse(from_os_str),
        short = "o",
        long = "output",
        required(true),
        help = "Set Out file name."
    )]
    pub out: std::path::PathBuf,

    #[structopt(name = "Encrypted file", short = "e", long = "encrypt", help = "If file encrypted use this flag")]
    pub encrypted: bool,

    #[structopt(parse(from_os_str), long = "public", required_if("encrypted", "true"))]
    pub pub_key_file: std::path::PathBuf,

    #[structopt(parse(from_os_str), long = "private", required_if("encrypted", "true"))]
    pub priv_key_file: std::path::PathBuf,

}
