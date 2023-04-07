use std::io;

fn main() {
    println!("Informe o valor da base");
    let mut base = String::new(); 
    io::stdin()
        .read_line(&mut base)
        .expect("Digite um valor numérico");

    let base: i32 = base.trim().parse().expect("Entrada inválida.");

    println!("Informe o valor da altura");
    let mut altura = String::new();
    io::stdin()
        .read_line(&mut altura)
        .expect("Digite um valor numérico");
    
    let altura: i32 = altura.trim().parse().expect("Entrada inválida.");
    
    let area = base*altura;
    
    println!("A área do retângulo informada é {area}!");
}