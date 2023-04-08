use std::io;

fn main() {
    /*5 - Dados três números, verificar se eles podem representar as medidas dos lados de um triângulo e, classificar o triângulo em equilátero, isósceles ou escaleno. Para que três números representem os lados de um triângulo é necessário que cada um deles seja menor que a soma dos outros dois. Um triângulo é equilátero se tem os três lados iguais, isósceles se tem apenas dois lados iguais e escaleno se tem todos os lados distintos.*/

    let mut lado1 = String::new();
    let mut lado2 = String::new();
    let mut lado3 = String::new();
    
    println!("Informe o lado 1:");
    io::stdin().read_line(&mut lado1).expect("Falha em ler a linha");
    let lado1: u32 = match lado1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Informe um valor numérico válido");
            return;
        }
    };

    println!("Informe o lado 2:");
    io::stdin().read_line(&mut lado2).expect("Falha em ler a linha");
    let lado2: u32 = match lado2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Informe um valor numérico válido");
            return;
        }
    };

    println!("Informe o lado 3:");   
    io::stdin().read_line(&mut lado3).expect("Falha em ler a linha");
    let lado3: u32 = match lado3.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Informe um valor numérico válido");
            return;
        }
    };
    if (lado1+lado2>lado3) && (lado1+lado3>lado2) && (lado2+lado3>lado1){
        if lado1==lado2 && lado2==lado3{
            println!("O triângulo é equilátero!");
        }
        else if lado1==lado2 || lado1==lado3 || lado2==lado3 {
            println!("O triângulo é isóscelos!");
        }
        else {
            println!("O triãngulo é escaleno!");
        }
    }
    else{
        println!("Não representa um triângulo!")
    }
}
