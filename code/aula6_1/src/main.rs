use std::io;

fn main() {
    println!("Digite o valor do Multiplicador: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();

    for i in 1..=10 {
        println!("{} x {} = {}", number, i, number * i);
    }
}
