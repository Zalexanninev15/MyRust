fn main() {
    for i in 1..5 { // range без 5, от 1 до 4
        println!("{}", i);
    }
    println!();
    for i in 1..=5 { // range с включением 5, от 1 до 5
        println!("{}", i);
    }
    println!();
    for i in (1..=5).rev() { // range с включенным 5, от 1 до 5, но в обрабатном порядке (от 5 до 1) с помощью rev() (делать до collect())
        println!("{}", i);
    }
    println!();
    let numbers : Vec<i32> = (1..=10).collect(); // создание массива из диапазоне от 1 до 10 включительно
    for i in numbers {
        println!("{}", i);
    }
    println!();

    // шаг
    let numbers : Vec<i32> = (1..=20).rev().step_by(2).collect(); // создание массива из диапазоне от 1 до 20 включительно в обрабном порядке
    // и применение шага 2 (только чётные числа)
    for i in numbers {
        println!("{}", i);
    }
    println!();
    // слайс (срез)
    let numbers2 : Vec<i32> = (1..=20).rev().step_by(2).collect();
    let slice = &numbers2[2..7];
    for i in slice {
        println!("{}", i);
    }
    println!();
    
    for i in 'a' ..='z' {
        println!("{}", i);
    }
}
