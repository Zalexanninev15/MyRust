mod test;

use test::{test1}; // использование определённого функционала из модуля
use test::test1::hello as great; // импорт функционала из модуля под другими имененм или просто так

// mod test1 {
//     pub fn say_test() { // pub - публичный модификатор доступа
//         println!("Test");
//         hello();
//     }
//
//     fn hello() {
//         println!("Hello");
//     }
//
//     pub mod test1 {
//         pub fn hello() {
//             println!("Test 1");
//         }
//     }
//
//     pub mod test2 {
//         pub fn hello() {
//             println!("Test 2");
//         }
//     }
// }

fn main() {
    // Из данного файла, модуль test1
    // test1::say_test();
    // test1::test1::hello();
    // test1::test2::hello();

    // Из модуля test
    test1::hello(); // т.к. указан в use
    great();
    test::test2::hello(); // не указан в use


}
