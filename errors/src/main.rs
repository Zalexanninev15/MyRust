use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    // Обработка ошибок

    // 1. Использование макрокоманды panic!() (неустранимая ошибка)
    panic!("This is Error");
    // Принудительно "крашнуть" программу

    // 2.Тип Result<> (устранимая ошибка)
    let path = "text.txt";
    let f0 = File::open(path); // Открытие существующего файла

    // Переопределяем переменную f, новый результат берётся из match
    let f0 = match f0 {
        Ok(file) => file, // Передаём в переменную f файл (file)
        Err(e) => panic!("Error opening file: {:?}", e), // Ошибка открытия файла
    };

    // 3. Обработка конкретных ошибок с помощью match
    let f = File::open(path); // Открытие существующего файла

    // Переопределяем переменную f, новый результат берётся из match
    let f = match f {
        Ok(file) => file, // Передаём в переменную f файл (file)
        Err(e) => match e.kind() {
            /* match e.kind() - перечисление ошибок, e.kind() - вид ошибки; Обработка и определение ошибки открытия файла
            Создаём файл, если он ошибка "Не найден"*/
            ErrorKind::NotFound => match File::create(path) {
                Ok(file) => file, // Передаём в переменную f файл (file)
                Err(e) => panic!("Eror creating file: {:?}", e), // Ошибка создания файла
            },
            other => panic!("Error occured: {:?}", other),
        },
    };

    // 4. Методы unwrap() и expect()
    let f1 = File::open(path).unwrap();
    /* unwrap() берёт из Ok(...) => ... файл (file)
    Ok(f) => file,
    Err(e) => panic!("... {:?}", e),
    */
    let f1 = File::open(path).expect("My Error");
    // Тоже самое что и unwrap(), но можно добавить свой текст panic!

    // 5. Распространение ошибок
    match read_file_text(path) {
        Ok(data) => println!("File text: {}", data), // data из функции read_file_text
        Err(e) => panic!("Error occured: {:?}", e),
    }
    match read_file_text(path) {
        Ok(data) => println!("File text: {}", data), // data из функции read_file_text
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(_) => println!("File created!"),
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other => panic!("Error occured: {:?}", other),
        },
    }

    // 6. Оператор ? (в функции read_file_text_Q)
    match read_file_text_q(path) {
        Ok(data) => println!("File text: {}", data), // data из функции read_file_text
        Err(e) => panic!("Error occured: {:?}", e),
    }
    match read_file_text_q(path) {
        Ok(data) => println!("File text: {}", data), // data из функции read_file_text
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(_) => println!("File created!"),
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other => panic!("Error occured: {:?}", other),
        },
    }
    // Только в функицях, котрые возвращают Result
}

// Возвращение текста из файла или ошибки с ?
fn read_file_text_q(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;
    /* "?"" берёт из Ok(...) => ... файл (file)
    Возвращает ошибку туда, откуда вызывается функция (без panic!)
    */
    let mut data = String::new();
    // Считаем из файла данные и записываем их в переменную data
    f.read_to_string(&mut data)?;
    Ok(data)
}

// Возвращение текста из файла или ошибки
fn read_file_text(path: &str) -> Result<String, Error> {
    let f = File::open(path);
    // Обработка открытия файла
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut data = String::new();
    // Считаем из файла данные и записываем их в переменную data
    match f.read_to_string(&mut data) {
        Ok(_) => Ok(data), // Если Ok() -> возвращаем текст из файла
        Err(e) => return Err(e),
    }
}
