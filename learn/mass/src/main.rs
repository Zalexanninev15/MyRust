fn main() {
    // Массив
    let mut array = [1, 2, 3, 4, 5];
    array[2] = 4;
    println!("{}", array[2]);
    println!("{:?}", array);

    let arr = [2; 10]; // 2 будет повторятся 10 раз. 2 можно заменить на любой тип данных
    println!("{:?}", arr);

    // Перебор массива
    for i in arr.iter() {
        println!("{}", i);
    }

    println!("{}", array.len()); // длина массива
}
