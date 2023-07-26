enum Person {
    Adult,
    Underage   
}

enum Say {
    Hi(String), // Связывание типа данных
    Bye,
}

fn main() {
    let person = Person::Underage;

    match person {
        Person::Adult => println!("Заходи!"),
        Person::Underage => {
            println!("Вход запрещён!");
            println!("Ещё не дорос!");
        }
    }

    let say = Say::Hi("HELLO".to_string());
    match say {
        Say::Hi(_) => todo!(),
        Say::Bye => todo!(),
    }
}
