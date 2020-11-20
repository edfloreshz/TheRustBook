pub fn test_references() {
    let mut a = 0xff;
    { // This creates a scope and releases 
      // memory when it reaches the closing bracket
        let s = &mut a;
        //s = 2;
        println!("{}", s);
    }                                                                            
    let q = &mut a;
    println!("{}", q);
}
