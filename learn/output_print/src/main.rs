fn main() {
    let n1 = 22;
    let n2 = 10;
    println!("{}", n1 + n2);

    // Позиционные аргументы - изменение порядка элементов при выводе
    let a1 = 1;
    let a2 = 2;
    let a3 = 3;
    println!("{2} {0} {1}", a1, a2, a3);
    println!("{0} {0} {0}", a1);

    // Именованные аргументы
    println!(
        "{number1} {number2} {number3}",
        number1 = a1,
        number2 = a2,
        number3 = a3
    );

    // Вывод и конвертация чисел
    let s1 = 10;
    let s2 = 54;
    let s3 = 24;
    println!("Binary: {:b}\nOctal: 0o{:o}\nHex: 0x{:x}", s1, s2, s3);
    println!("\nBinary: {0:b}\nOctal: 0o{0:o}\nHex: 0x{0:x}", s1);

    // Вывод сложных типов данных
    let list = vec![1, 4, 5, 7, 9];
    let tuple = (77, 78.43, "Hello");
    println!("\n{:?}\n{:?}", list, tuple);
    // Форматированный вывод
    println!("\n{:#?}\n{:#?}", list, tuple);
    println!("\n{1:#?}\n{0:#?}", list, tuple);

    // eprint! и eprintln! - возникла ошибка (можно не использовать)
    eprintln!("Hello");
}
