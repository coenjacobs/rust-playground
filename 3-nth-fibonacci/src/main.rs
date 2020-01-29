use std::io;

fn main() {
    println!("Nth Fibonacci!");
    println!("Please enter what nth Fibonacci number you'd like to calculate:");

    let mut string_nth = String::new();

    io::stdin().read_line(&mut string_nth)
        .expect("Failed to read line, bye!");

    let nth: u64 = string_nth.trim().parse().unwrap();
    let fibon: u64 = calc_fibon(nth);

    println!("{0}: {1}", nth.to_string(), fibon.to_string());
}

fn calc_fibon(nth :u64) -> u64 {
    match nth {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => calc_fibon(nth - 1) + calc_fibon(nth - 2),
     }
}
