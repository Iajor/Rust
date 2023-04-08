use std::io; //importa a lib io de dentro da std
use rand::Rng; //para criar o número secreto aleatoriamente
use std::cmp::Ordering; //para comparar os números

fn main() {
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100); //chama a função thread_rng, que já fornece uma semente para o gerador de números aleatórios com base em dados do sistema, e gen_range é um método que retorna um número aleatório dentro do intervalo especificado, sendo que para ser inclusivo o final deve ter "=" 

    let mut tentativas=1; //total de tentativas

    loop { //loop para gerar mais chances de jogar

        println!("Tentativa {tentativas} de 10");

        println!("Por favor, informe o seu chute:");
    
        let mut chute = String::new(); //os inputs são strings

        io::stdin()
        .read_line(&mut chute) //lê o input e atribui o valor ao endereço da memoria vinculado a variável chute
        .expect("Falha em ler a linha"); // caso dê erro, "Err"

        let chute: u32 = match chute.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //como os inputs são strings é necessário converter o chute para um número já que o número secreto será um número e código não faz comparação entre strings e números. trim() elimina espaços em branco no início e no fim, e parse() converte a string para outro tipo considerando a anotação no começo "u32", retornando um valor tipo Result sendo Ok ou Err. Alguns input não podem ser convertidos.
        }; // o {} está relacionado ao match do início que verifica se o chute é um chute válido para que o jogador possa continuar a jogar sem interromper o programa. Se o parse retorna um valor Ok, match retorn o valor num que resultou. Se retorna um valor Err, o código continua.

        match chute.cmp(&numero_secreto) { //o método .cmp faz a comparação entre os valores e retorna uma das três possibilidades abaixo com base no Ordering e então o match decide o que faz em seguida
            Ordering::Less => println!("Seu chute é menor que o número secreto!"),
            Ordering::Greater => println!("Seu chute é maior que o número secreto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
        tentativas+=1;//no final de cada loop aumenta a tentativa em 1
        if tentativas==11 { // no final de cada loop verifica se o jogador não ultrapassou o número de tentativas
                println!("Você perdeu!");
                break;
            }
        
    }
}
