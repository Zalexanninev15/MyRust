fn main() {
    // Функции
    wallet_info(String::from("Lorax"), 23, 2000.543);
    let mult = mul(4, 2);
    println!("{}", mult);
    let (sum, min, multt, msg) = math(4, 6);
    println!("{}, {}, {}, {}", sum, min, multt, msg);
}

fn wallet_info(name: String, age: i32, wallet: f64) {
    println!("{} is {} years old, wallet: ${}", name, age, wallet);
}

// Возвращение умножения a на b в формате int32
fn mul(a: i32, b: i32) -> i32 {
    a * b // или return a * b;
}

// Возвращение в формате int32, int32, int32
fn math(a: i32, b: i32) -> (i32, i32, i32, String) {
    (a + b, a - b, a * b, String::from("Good!"))
}
