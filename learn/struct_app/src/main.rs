// Аннотация
#[derive(Debug)]
struct Person { 
    name : String,
    surname : String,
    age: i32,
    balance: f64,
}

#[derive(Debug)]
struct Str(i32, String, f64); // Обычный кортеж в виде струткуры

fn main() {
    let mut person1 = Person {
        name : String::from("Kate"),
        surname : String::from("Surname"),
        age : 25,
        balance : 384.36,
    };
    person1.surname = String::from("Name2");
    println!("{:?}", person1);
    println!("{:#?}", person1); // Упорядоченный вид
    println!("Имя: {}", person1.name); // Вывод имени
    println!("Фамилия: {}", person1.surname); // Вывод фамилии (изменённой)
    
    // Если название в структуре = названию переменной, то можно значение не писать
    let name = String::from("John");
    let surname = String::from("Smith");
    let person2 = Person {
        name,
        surname,
        age : 32,
        balance : 342.32
    };

    let person2 = Person {
        name : "Lol".to_string(),
        surname : "Lol".to_string(),
        ..person1 // Из person1 копируется age и balance
    };

    let object = Str(5, "TEXT".to_string(), 434.24);
    println!("{}", object.2); // 434.24
}