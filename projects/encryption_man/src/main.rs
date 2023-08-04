use std::io::{Read, Write};
use std::num::IntErrorKind;

fn get_input(query: &str) -> String {
    println!("{query}");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned() // т.к. возвращаем строку
}

fn process_file_data(data: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut process_data = Vec::with_capacity(data.len());
    for byte in data {
        process_data.push(byte ^ key);
    }
    process_data
}

fn main() {
    loop {
        println!("**** Encryption Man ****");
        // XOR ШИФРОВАНИЕ
        let input_filename = get_input("Enter file name to process: ");
        let input_file = match std::fs::File::open(&input_filename) {
            Ok(file) => file,
            Err(err) => {
                println!("Can't open file \"{input_filename}\": {err}\n");
                continue
            }
        };

        let key = match get_input("Enter a key for file encryption/decryption: ").parse::<u8>() {
            Ok(key) => key,
            Err(err) => {
                match err.kind() {
                    IntErrorKind::Empty => println!("Key mustn't be empty!\n"),
                    IntErrorKind::InvalidDigit => println!("Enter correct number!\n"),
                    IntErrorKind::PosOverflow => println!("Number must be in range [0-255]! (don't use 0 as key)\n"),
                    _ => println!("Error getting key!\n")
                };
                continue
            }
        };

        if key == 0 {
            println!("Key mustn't be 0!\n");
            continue
        }

        let mut reader = std::io::BufReader::new(input_file);
        let mut input_data = Vec::new();

        if let Err(err) = reader.read_to_end(&mut input_data) {
            println!("Error reading file: {err}\n");
            continue
        }

        let process_data = process_file_data(&input_data, key);

        let save_file_name = get_input("Enter file name to save result: ");
        let save_file = match std::fs::File::create(&save_file_name) {
            Ok(file) => file,
            Err(err) => {
                println!("Can't create file \"{save_file_name}\": {err}\n");
                continue
            }
        };

        let mut writer = std::io::BufWriter::new(save_file);
        if let Err(err) = writer.write_all(&process_data) {
            println!("Error writing file: {err}\n");
            continue
        }
        println!("Done!");
        println!("\n");
    }
}
