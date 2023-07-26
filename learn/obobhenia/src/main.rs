// Обобщения - заменитель типов данных
// Обобщения можно использовать для одной и той же функции, но которая принимает или/и отдаёт разные типы данных

use std::fmt::Display;

fn main() {
    let s = String::from("Здравствуйте");
    let num = 777;
    print_value(s);
    print_value(num);

    let nums = [ 10, 3, 2, 3, 7, 5, 7, 7, 8, 9, 10];
    println!("{:?}", find_duplicate_for_all(&nums));

    let chars = ['x', 'a', 'b', 'c', 'x', 'b'];
    println!("{:?}", find_duplicate_for_all(&chars));
}

fn print_value<T: Display>(value: T) {
    print!("{}", value);
}

fn find_duplicate_for_all<T: Display>(list: &[T]) -> Vec<T>
// Типажи для типа T
where T: PartialEq + Copy // Чтобы сравнивать разные типы данных
{
    let mut dubplicates = Vec::new();
    for i in 0..list.len() { // Проходим по всем элементам списка
        for j in i+1..list.len() {
            if list[i] == list[j] { // Нашли совпадение
                if !dubplicates.contains(&list[i]) { // Проверка, чтобы не было дубликатов в списке дубликатов (при 3+ совпадений)
                    dubplicates.push(list[i]);
                }
            }       
        }
    }
    dubplicates
}
