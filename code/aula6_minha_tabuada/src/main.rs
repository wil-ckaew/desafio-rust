/*
Aqui está uma breve explicação das mudanças feitas:

Loop Infinito com loop: Utilizamos um loop infinito para continuar perguntando ao usuário se ele deseja calcular outra tabuada.
Entrada de Dados do Usuário: Perguntamos qual tabuada o usuário deseja ver e lemos a entrada.
Cálculo e Exibição da Tabuada: Calculamos e exibimos a tabuada como no código original.
Pergunta de Continuação: Perguntamos ao usuário se ele deseja calcular outra tabuada.
Condição para Encerrar o Loop: Se o usuário responder qualquer coisa diferente de "s" (ou "S"), o loop se encerra e o programa termina.
Este código é simples e deve funcionar conforme esperado para calcular múltiplas tabuadas até que o usuário decida parar.
*/
use std::io;

fn main() {
    loop {
        println!("Qual tabuada você gostaria de ver? ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let number: i32 = input.trim().parse().unwrap();

        println!("Tabuada de {}:", number);
        for i in 1..=10 {
            println!("{} x {} = {}", number, i, number * i);
        }

        println!("Deseja calcular outra tabuada? (s/n)");
        let mut resposta = String::new();
        io::stdin().read_line(&mut resposta).unwrap();
        let resposta = resposta.trim();

        if resposta.to_lowercase() != "s" {
            break;
        }
    }
}
