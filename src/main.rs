use ciphers::{Cipher, Vigenere};
use dotenv::dotenv;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::BufRead;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Write};
use std::path::Path;
use std::string::String;


const PASSWORD_FILE_NAME: &str = "password.txt";

// Adds a new password to the file
fn add_password(name: &str, password: &str) -> Result<(), Error> {
    let cipher = get_cypher();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(PASSWORD_FILE_NAME)?;
    let encrypted_password = cipher.encipher(password.trim()).unwrap();

    writeln!(file, "{}: {}", name, encrypted_password)?;
    Ok(())
}
//deletes a password with a given name
fn delete_password(name: &str) -> Result<(), Error> {
    let passwords = read_passwords()?;
    let mut file = File::create(PASSWORD_FILE_NAME)?;
    let mut writer = BufWriter::new(&mut file);
    for (n, password) in passwords {
        if n != name {
            writeln!(writer, "{}: {}", n, password)?;
        }
    }
    Ok(())
}
//reads the full file of passwords
fn read_passwords() -> Result<Vec<(String, String)>, Error> {
    let file = File::open(PASSWORD_FILE_NAME)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut result = Vec::new();
    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let parts: Vec<&str> = buffer.trim().split(": ").map(|s| s.trim()).collect();
                if parts.len() != 2 {
                    return Err(Error::new(ErrorKind::Other, "Invalid file format"));
                }
                result.push((parts[0].to_owned(), parts[1].to_owned()));
            }
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}

fn get_cypher() -> Vigenere {
    let cipher = Vigenere::new(std::env::var("KEY").expect("Need the key").as_str());
    return cipher;
}
fn check_if_file_exists() {
    let path = Path::new(PASSWORD_FILE_NAME);
    if !path.exists() {
        let mut file = File::create(PASSWORD_FILE_NAME).unwrap();
        write!(file, "wb name|Pass").unwrap();
    }
}
fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    check_if_file_exists();



    loop {
        //simple menu
        println!("Usage:");
        println!("1  add <username> <password>   - Add a new password for the given username");
        println!("2  delete <username>          - Delete the password for the given username");
        println!("3  update <username> <password> - Update the password for the given username");
        println!("4  get <username>             - Get the password for the given username");
        println!("5  quit                        - Quit the program");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        let option_number: u32 = option.trim().parse().expect("number");
        match option_number {
            1 => {
                let mut username = String::new();
                let mut password_str = String::new();
                println!("please enter the name");
                io::stdin().read_line(&mut username).unwrap();
                println!("please enter the password");
                io::stdin().read_line(&mut password_str).unwrap();
                add_password(&username.trim().to_string(), &password_str.trim().to_string())?;
            }
            2 => {
                let mut username = String::new();
                println!("please enter the name");
                io::stdin().read_line(&mut username).unwrap();

                delete_password(&username)?;
            }
            3 => {
                // Get the username and new password from the user input
                let mut username = String::new();
                let mut new_password_str = String::new();
                println!("please enter the name");
                io::stdin().read_line(&mut username).unwrap();
                println!("please enter the new password");
                io::stdin().read_line(&mut new_password_str).unwrap();

                delete_password(&username)?;
                add_password(&username.trim().to_string(), &new_password_str.to_string())?;
            }
            4 => {
                let cipher = get_cypher();

                // Get the password for the given username from the password data
                let password_data = read_passwords().unwrap();

                for (name, password) in password_data {
                    println!("{}: {:?}", name, cipher.decipher(&password));
                }
            }
            5 => {
                println!("Goodbye!");
                return Ok(());
            }
            _ => {
                println!("Invalid option. Please choose a valid option:");
                continue;
            }
        }
    }
}
