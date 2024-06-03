use std::io;
use chrono::{DateTime, Utc, Datelike};

fn main() {
    /* 
    Dado que eu tenha um ano de nascimento, e faço a subtração pelo ano autal, 
    Então devo ter o valor da idade da pessoa
    */

    println!("Digite o ano que você nasceu: ");

    //Aqui capturei o ano em string
    let mut ano_string = String::new();
    io::stdin().read_line(&mut ano_string).expect("Falha ao ler entrada");
    //Aqui fiz a conversão do dado que veio por String para i32
    let ano_usuario: i32 = ano_string.trim().parse().expect("Por favor, digite um número");

    //Depois pegando o ano atual usando a biblioteca do chromo
    let ano_atual: i32 = Utc::now().year();
    //Crie a idade do usuario
    let idade_usuario: u8 = (ano_atual - ano_usuario) as u8;//aqui fiz a conversão de i32 as u8

    println!("A sua idade é: {}", idade_usuario);
}
