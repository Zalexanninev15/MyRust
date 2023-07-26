fn is_even(a: i32) -> bool {
    a % 2 == 0
}

// Замыкание - анонимная функция, которую можно сохранить в переменную и потом вызывать из этой переменной

fn main() {
    // Обычная функция
    println!("{}", is_even(8));

    // Замыкание
    let is_even_closure = |a: i32| a % 2 == 0;
    println!("{}", is_even_closure(7));

    // Возвращаемый тип данных + типа функция
    let is_even_closure = |a: i32| -> bool {
        println!("{}", a);
        a % 2 == 0
    };
    println!("{}", is_even_closure(4));

    // Замыкание с несколькими параметрами
    let add = |a: i32, b: i32| a + b;
    println!("{}", add(7, 10));

    // Замыкание без параметров
    let x = || println!("Empty closure");
    x();

    // Замыкание копии переменной
    let mut num = 8;
    let add2 = move |x: i32| x + num;
    let y = &mut num;
    println!("{}", add2(5));
}
