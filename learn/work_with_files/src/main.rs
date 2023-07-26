use std::fs::{File, OpenOptions};
use std::io::{Read, stdin, Write};

fn main() {
    // Методы открытия и создания файла
    File::create("text.txt").expect("Error creating file!"); // Создание файла или удаляет данные если он существует
    File::open("text.txt").expect("Error opening file!"); // Открытие существующего файла (не создаёт)

    // Запись данных в файл
    let path = "data.txt";
    let mut file = File::create(path).expect("Error creating file");
    file.write_all("Hello from Zalexanninev15".as_bytes()).expect("Error writing to file!"); // Запись всего текста в файл с перезаписью всего (file должен быть mut), string -> as_bytes()
    // Или string -> bytes: b"Hello from Zalexanninev15"

    // Чтение данных из файла
    let mut file_read = File::open(path).expect("Error opening file");
    let mut file_data = String::new(); // Обязательно mut
    file_read.read_to_string(&mut file_data).expect("Error reading file"); // Чтение текста из файла, &mut
    println!("{}", file_data);

    // Режим read-write при открытии файла (кастомный режим открытия файла)
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Просто создаёт новый файл или откроет существующий без удаления данных
        .open(path)
        .expect("Error opening/creating file");
    let mut file_data1 = String::new();
    f.read_to_string(&mut file_data1).expect("Error reading file");
    println!("File data: {}", file_data1);
    println!("Введите что-то: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error getting user input");
    f.write_all(input.as_bytes()).expect("Error writing to file");

    // Переименование файла
    std::fs::rename("text.txt", "test.txt").expect("Error");

    // Удаление файла
    std::fs::remove_file(path).expect("Error");
}
