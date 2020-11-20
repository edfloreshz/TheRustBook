pub fn test_lifetime() {
    let string1 = String::from("long string");
    let res;
    {
        // This gives an error because --------------------------
        let string2 = String::from("short"); //                 |
        res = longest(string1.as_str(), string2.as_str()); //   |
    } // string2 is destroyed after this line. <-----------------
    // println!("{}", res);

    // This works because &str has a 'static lifetime, which means
    // it will live as long as the program does.
    let string1 = "long string";
    let res;
    {
        let string2 = "short";
        res = longest(string1, string2);
    }
    println!("{}", res);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
