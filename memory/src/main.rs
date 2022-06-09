fn main() {
    // Владение
    // let num = 10; // Владелец - num
    // if num == 10 {
    //     let str = Strng::from("Hello!");
    // }
    
    // Хранится в стеке
    // Копируется целые числа, bool, float, char, кортежи (которые не содержат string)
    let a = 1;
    let b = a; // a копируется в b
    println!("{}, {}", a, b);

    // Хранится в куче (операции требуют больше места, работают медленнее)
    let str1 = String::from("TEXT");
    // let str2 = str1; // Данные не копируется, а перемещаются. Переменной str1 больше нет
    // Чтобы скопировать надо:
    let str2 = String::from(str1.clone());
    println!("{}, {}", str1, str2);


    // Владение функцией
    let str3 = String::from("TEXT");
    print_value(str3.clone());
    println!("{}", str3);
}

fn print_value(s: String) {
    println!("{}", s);
}