use std::io;

fn main() {
    let input = user_input();
    println!("The n-th fibonacci's number is {}",fibonacci(input));

}

fn user_input() -> u64{
    loop{
        println!("Please insert your input: \n");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
        .expect("Error reading the line!");

        let input: u64 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        return input;
    } 

}


fn fibonacci(x: u64) -> u64{
    if x == 0{
        return 0;
    }
    let mut a : u64 = 0;
    let mut b : u64 = 1;

    for _ in 1..x{
        let next = a + b;
        a = b; 
        b = next;
    }
    b
}
