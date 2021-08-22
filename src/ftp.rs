use ftp4::FtpStream;
use std::io::Cursor;

pub struct FtpFile<'a> {
    pub content: &'a Vec<u8>,
    pub name: &'a String,
}

pub struct FtpClient<'a> {
    ftp: FtpStream,
    username: &'a String,
    password: &'a String,
}

impl<'a> FtpClient<'a> {
    /// Create a new ftp client
    pub fn new(server_addr: &String, username: &'a String, password: &'a String) -> Self {
        let ftp = FtpStream::connect(server_addr).unwrap();

        Self {
            ftp: ftp,
            username: username,
            password: password,
        }
    }

    /// Upload file to a ftp server
    pub fn upload(&mut self, file: &FtpFile) -> () {
        // First login to ftp server
        match &self.ftp.login(&self.username, &self.password) {
            Err(error) => {
                println!("Error: {}", error);
            }
            _ => {}
        }

        // Get the current directory that the client will be reading from and writing to.
        println!("Current directory: {}", &self.ftp.pwd().unwrap());

        // Start Uploading file to server
        let mut reader = Cursor::new(file.content);

        // Now upload file to server
        let _ = &self.ftp.put(file.name, &mut reader);
        println!("File uploaded to server!");

        // Terminate the connection to the server.
        let _ = &self.ftp.quit();
    }

    /// Donwload from ftpServer
    pub fn download(&mut self, filename: &String) -> Vec<u8> {
        // First login to ftp server
        match &self.ftp.login(&self.username, &self.password) {
            Err(error) => {
                println!("Error: {}", error);
            }
            _ => {}
        }

        // Get file cursor
        let file = self.ftp.simple_retr(&filename).unwrap().into_inner();
        println!("File Downloaded from server.");

        file
    }
}
