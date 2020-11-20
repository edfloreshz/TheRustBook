pub fn generate_sequence(mut fib_nums: Vec<u32>, limit: u32){
    for n in 0..limit {
        &fib_nums.push(fib_nums[n as usize] + fib_nums[n as usize +1]);
    }
    for n in fib_nums.iter() {
        print!("{}, ", n)
    }
}
