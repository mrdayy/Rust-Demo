use std::io;

fn main() {
    println!("Masukkan todo kamu:");

    let mut todo = String::new();
    io::stdin().read_line(&mut todo).unwrap();

    println!("Todo kamu: {}", todo.trim());
}