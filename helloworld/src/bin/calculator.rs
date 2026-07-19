use std::io;

fn main() {
    println!("|-------------------|");
    println!("|     CALCULATION   |");
    println!("|                   |");
    println!("| X   +    -     /  |");
    println!("|-------------------|");

    calculation();
}

fn calculation() {
    let mut a = String::new();
    let mut b = String::new();
    let mut option = String::new();
    println!("Write number:1");
    io::stdin()
        .read_line(&mut a)
        .expect("FAILED TO READ THE LINE");
    println!("Write number 2");
    io::stdin()
        .read_line(&mut b)
        .expect("FAILED TO READ THE LINE");

    let a: usize = a.trim().parse().expect("Not an number");
    let b: usize = b.trim().parse().expect("Not an number");

    println!("What do you want to calculate (press 1 - 4)");
    println!("1. Addition \n2. Multiplication \n3. Subtraction \n4. Division");
    io::stdin()
        .read_line(&mut option)
        .expect("FAILED TO READ THE LINE");

    let option: u32 = option.trim().parse().expect("Not an number");

    fn subtraction(a: usize, b: usize) -> usize {
        if a < b { b - a } else { a - b }
    }

    match option {
        1 => println!("The answer is {}", a + b),
        2 => println!("The answer is {}", a * b),
        3 => println!("The answer is {}", subtraction(a, b)),
        4 => println!("The answer is {}", a / b),
        _ => println!("The answer is {}", a + b),
    }
}
