pub fn iter() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    for val in v2.iter() {
        println!("{}", val);
    }
}
