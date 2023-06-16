// While let - тоже самое, что и if let, но работает в цикле while

fn main() {
    // Пример 1
    let mut counter = Some(0);
    while let Some(n) = counter {
        if n == 31 {
            println!("End");
            counter = None;
        } else {
            println!("{}", n);
            counter = Some(n + 1);
        }
    }
    // loop {
    //     match counter {
    //         Some(i) => {
    //             if i == 31 {
    //                 println!("End");
    //                 counter = None;
    //             } else {
    //                 println!("{}", i);
    //                 counter = Some(i + 1);
    //             }
    //         }
    //         None => break,
    //     }
    // }

    // Пример 2
    let data = vec![
        vec!["Den", "11", "10", "12"],
        vec!["Kate", "8", "4", "9"],
        vec!["John", "6", "7", "2"],
    ];
    for mut students in data {
        println!("For {}", students[0]);
        while let Some(value) = students.pop() {
            // Если то, что спарсили, можно сконвертировать в число
            if let Ok(result) = value.parse::<i32>() {
                println!("{}", result * 2);
            } else {
                println!("End");
            }
        }
    }
}
