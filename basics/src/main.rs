use std::io; // Импорт для использования функции stdin для ожидания ввода

fn main() {
    // mut - переменную можно изменять
    // String
    let lang = "Rust"; // Определение string
    let mut text = String::from("Max"); // Правильное определение string
    text = "1".parse().unwrap(); // Изменение значения переменной
    print!("{}", text); // Обычный вывод текста
    println!("Hello world, {}", lang); // Ввывод текста, потом с новой строки

    // Int64
    let age: i64 = 23; 
    println!("My age is {}", age);

    // Float
    let float: f64 = 134.65;
    println!("Number is {}", float);

    // Char
    let symbol: char = 'c';
    let smile: char = '🤔';
    println!("Char {} is char {}", symbol, smile);

    // Bool
    let ll_t: bool = true;
    let ll_f: bool = false;
    println!("{} - {}", ll_t, ll_f);

    // Логика
    let is_true: bool = true;
    let num = if is_true {
        1
    } else {
        0
    };
    println!("{}", num);

    // Работа с функцией
    // let lol = String::from(funct());
    // println!("{}", lol);

    // Работа с функцией i64
    let result = funct_sum(4, 9);
    println!("{}", result);

    // Бесконечный цикл
    let mut num = 0;
    loop {
        println!("{}", num);
        num += 1;
        if num == 100 { break; }
    }

    // Цикл
    let mut counter = 0;
    while counter < 10 {
        println!("Counter is {}", counter);
        counter += 1;
    }

    // For
    for i in 0..101 { // >= 0 and < 101
        if i % 2 == 0 { // Деление нацело, остаток == 0
            println!("{}", i)
        }
    }

    // Switch
    let num = 24;
    match num {
        10 => println!("Num is 10!"),
        23 => { 
            println!("Num is 45!");
            println!("OK!"); 
        },
        10..=50 => { // число между 10 и 50
            println!("Число между 10 и 50");
        },
        _ => { // _ - если ничего из вышеперечисленного не верно
            println!("Таких вариантов нет!");
        }
    };

    // Switch с присваиванием
    let numx = 3;
    let num = match numx {
        2 => 1,
        3 => 10,
        3..=10 => 7,
        _ => 0
    };
    println!("{}", num);

    // Ввод
    let mut name = String::new();
    println!("Введите свой имя: ");
    match io::stdin().read_line(&mut name) { // Проверка на ошибки
        Ok(_) => { // Можно так Ok(_) => { }, 
            println!("Привет, {}", name)
        },
        Err(e) => {
            println!("Ошибка!");
        }
    }
    io::stdin().read_line(&mut name).unwrap();
}

fn funct()
-> &'static str {
    // println!("LoL");
    "LoL"
}

fn funct_sum(x: i64, y: i64) 
-> i64 {
    x + y
}
