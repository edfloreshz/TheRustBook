pub fn get_largest() {
    let numbers = vec![34, 45, 6, 7, 88, 12];
    let result = largest(&numbers);
    println!("{}", result);

    let numbers = vec!['a', 'v', 'd', 't', 'u', 'w'];
    let result = largest(&numbers);
    println!("{}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }
    largest
}
