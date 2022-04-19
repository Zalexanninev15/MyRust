use std::io;

fn main() {
    // ax^2 + bx + c = 0
    
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();
    
    println!("Введите квадратное уравнение");
    println!("Введите 'a':");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err(e) => println!("Ошибка ввода - {}", e)
    }
    println!("Введите 'b':");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err(e) => println!("Ошибка ввода - {}", e)
    }
    println!("Введите 'c':");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err(e) => println!("Ошибка ввода - {}", e)
    }

    // String => Float
    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();
    println!("Введено уравнение: {}x^2+{}x+{}", a, b, c);
    let d: f64 = (b*b)-4.0*(a*c);
    
    if d > 0.0 {
        let x1 = ((-b)+d.sqrt())/(2.0*a);
        let x2 = ((-b)-d.sqrt())/(2.0*a);
        println!("Решено\nЕсть 2 корня: D = {}\nКорень 1 = {}\nКорень 2 = {}", d, x1, x2);
    }
    
    if d == 0.0 {
        let x = (-b) / (2.0 * a);
        println!("Решено\nЕсть 1 корень: D = 0\nКорень = {}", x);
    }

    if d < 0.0 {
        println!("Корней не существует!\nD < 0");
    }
}