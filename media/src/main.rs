use std::io;

fn main() {
    println!("Informe um valor:");
    let mut num1 = String::new(); 
    io::stdin()
        .read_line(&mut num1)
        .expect("Digite um valor numérico");

    let num1: f32 = num1.trim().parse().expect("Entrada inválida.");

    println!("Informe outro valor:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Digite um valor numérico");
    
    let num2: f32 = num2.trim().parse().expect("Entrada inválida.");
    
    let media = (num1+num2)/2.0;
    
    println!("A média dos valores é {media}!");
}