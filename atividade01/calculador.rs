use std::io::{self, Write};

fn main() {
    println!("Ola, mundo!");

    println!("Digite um numero para ver a tabuada: ");

    let mut entrada = String::new();
    let _ = io::stdin().read_line(&mut entrada);

    let numero = match entrada.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Entrada invalida ou vazia");
        }
    };

    println!("\nTabuada do {}:", numero);
    for i in 1..=10 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}
