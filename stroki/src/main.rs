fn main() {
    let s1 = String::new(); // Создание строки
    let mut s2 = String::from("Hello"); // Строка с значением (1 способ)
    let mut s3 = "Rust".to_string(); // Строка со значеним (2 способ)
    s3.push_str(" World!"); // Добавление к string
    println!("{}", s3);
    s2.push('!'); // Добавление char к string
    println!("{}", s2);
    let mut res = s3 + &s2; // Соединение строк
                            // или format!("{} {}", s3, s2);
    println!("{}", res);

    // &str - строковый литерал

    // "Индексирование строк" (нарезка строк)
    let mut hello = "Hello".to_string();
    println!("Size: {}", hello.len()); // Размер строки, 1 символ АНГЛ = 1 байт
    println!("{}", &hello[..1]); // H из Hello
    hello = "Привет".to_string();
    println!("Размер: {}", hello.len()); // Размер строки, 1 символ РУС = 1 байт
    println!("{}", &hello[..2]); // П из Привет

    // Перебор строк
    let s_char = "TEXT".to_string();
	
    // 1 способ - по символам
    for ch in s_char.chars() {
        print!("{} ", ch);
    }
    println!("");
	
    // 2 способ - по байтам
    for b in s_char.bytes() {
        print!("{} ", b);
    }
	
    // Байты для русского языка
    let ru_s_char = "ТЕКСТ".to_string();
    println!("");
    for b in ru_s_char.bytes() {
        print!("{} ", b);
    }
}
