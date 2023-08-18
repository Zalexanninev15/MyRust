mod subtest;

pub fn say_test() {
    // pub - публичный модификатор доступа
    println!("Test");
    hello();
}

fn hello() {
    println!("Hello");
}

pub mod test1 {
    // use crate::test::subtest::subtest; // Для варианта 1

    pub fn hello() {
        println!("Test 1");
        // subtest(); Вариант 1
        super::subtest::subtest();// область видимости для родительского модуля; Вариант 2
    }
}

pub mod test2 {
    pub fn hello() {
        println!("Test 2");
    }
}
