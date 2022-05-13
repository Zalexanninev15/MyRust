fn main() {
    // Кортежи
    // Массив с круглыми скобками и с разными типами данных внутри

    let tuple = (12, 35.7, String::from("Kitty"));
    println!("{:?}", tuple);

    // Получение значений из кортежа
    let chel = ("Max", 5);
    // Вариант 1
    let (name, grave) = chel;
    println!("Человек: {}, оценка - {}", name, grave);
    // Вариат 2
    println!("{}, {}", chel.0, chel.1);
}