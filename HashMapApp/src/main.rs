use std::collections::HashMap;

fn main() {
    // HashMap
    let mut map = HashMap::new();
    map.insert("Max".to_string(), 5);
    map.insert("Kate".to_string(), 11);
    map.insert("Jonh".to_string(), 7);
    println!("{:?}", map);

    // Владение HashMap
    let mut new_map = HashMap::new();
    let n1 = "Max".to_string();
    let n2 = "Kate".to_string();
    let n3 = "Jonh".to_string();
    new_map.insert(&n1, 5);
    new_map.insert(&n2, 11);
    new_map.insert(&n3, 7);
    // Перемещённые в HashMap значения без "&"" более нельзя использовать

    // Получение элементов из HashMap

    // Способ 1
    println!("{}", map["Max"]); // 5
    println!("{}", new_map[&n2]); // 11

    // Способ 2
    match map.get(&n1) {
        Some(mark) => {
            println!("Mark is {}", mark); // 5
        }
        None => {
            println!("Element doesn't exist!")
        }
    }

    // Перебор значений (не по порядку, рандомно)
    for (name, mark) in &map {
        println!("{} has {}", name, mark)
    }

    // Изменение HashMap
    let n4 = "Vlad".to_string();
    new_map.entry(&n4).or_insert(9); // Vlad со значением 9 (значения прошлые не перезаписываются)
    println!("{:?}", new_map);

    // Пример работы для HashMap. Подсчёт количества совпадений слова
    let s = "Turkish Angora My Dog dog".to_lowercase();
    // let s = "Turkish Angora My Dog Dog";
    let mut count_map = HashMap::new();
    // split_whitespace - разделение строки на слова. В качестве разделителя используется пробел
    for w in s.split_whitespace() {
        let count = count_map.entry(w).or_insert(0);
        *count += 1;
        /*
        Вставляем значение - слово
        Т.к. слово уже найдено, то count+1 и записывается в HashMap
        Если слово повторяется, то count+1  и записывается в HashMap
         */
    }
    println!("{:?}", count_map);
}
