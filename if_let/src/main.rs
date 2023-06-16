// if let - то же самое, что и match, но писать надо меньше
// Рекомендуется всё же использовать match, а не if let

enum Num {
    One(String),
    Two(String),
    Three(String),
}

struct User {
    name: String,
    age: i8,
}

fn main() {
    let nums = (1, 2, 3, 4, 5); // Кортеж
    if let (1, 2, 3, 4, 5) = nums {
        // Сравнивать кортежи разных размеров нельзя
        println!("Yes");
    } else {
        println!("No");
    }

    // if let (1, 2, a, s, d) = nums {
    //     Если видит, что первые элементы до "d" равны, то всё хорошо (но есть предупреждение)
    //     println!("Yes");
    // } else {
    //     println!("No");
    // }

    let t = Num::Two("2".to_string());
    /* match - для множества вариантов
    match t {
        Num::One(s) => println!("{}", s),
        Num::Two(s) => println!("{}", s),
        Num::Three(s) => println!("{}", s),
    }
    if let - для небольшого множества вариантов */
    if let Num::One(s) = t {
        println!("{}", s);
    } else {
        println!("Not One!");
    }

    let n = "John".to_string();
    let user = User { name: n, age: 30 };
    if let User { name: n, age: _ } = user {
        println!("Matched!");
    }
    // Здесь else не будет вызван
}

/*
if let шаблон = выражение {
    код...
} else {
    код...
}
*/
