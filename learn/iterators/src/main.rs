fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    // find + Some(...)
    println!(
        "find: {:?}",
        numbers.iter().find(|num| num == &&3)
    );

    // iter + map (with action) + collect
    let nm: Vec<_> = numbers
        .iter()
        .map(|num| num * 2)
        .collect();
    println!("iter + map (with action) + collect: {:?}", nm);

    // filter
    let nm2: Vec<_> = numbers
        .iter()
        .map(|num| num * 2)
        .map(|num| num - 1)
        .filter(|num| num % 3 == 0)
        .collect();
    println!("filter: {:?}", nm2);

    // По такому же принципу: count() -> int..., last() -> Some(...), max(), min()

    // sum
    println!("sum: {:?}", numbers.iter().sum::<i32>());

    // take - берём первые n элементов
    let s: Vec<_> = numbers
        .iter()
        .take(3)
        .map(|num| num * num)
        .collect();
    println!("take: {:?}", s);
    // skip() работает также

    // zip
    let numbers2 = vec![7, 8, 9];
    let x: Vec<_> = numbers.iter().zip(numbers2.iter()).collect();
    for i in x.iter() {
        println!("{:?}", i);
    }
    /* 
    Example:
    (1, 7)
    (2, 8)
    (3, 9)
    */

    let characters = vec!['a', 'b', 'c', 'x'];
    let x2: Vec<_> = numbers.iter().zip(numbers2.iter()).zip(characters.iter()).collect();
    for i in x2.iter() {
        println!("{:?}", i);
    }
    /*
    Example:
    (1, 7, 'a')
    (2, 8, 'b')
    (3, 9, 'c')
     */
}
