/*
exercicio da calculadora
pedir pra digitar um numero par multiplicador
exemplo
multiplicador =2
e mostra o resultado
2 x 1 = 2
2 x 2 = 4
2 x 3 = 6
*/
fn main() {
    /*não precisa descrever esta variavel senão aloca espaço na memoria
     e vc rescreve ela la embaixo senão fica duas vezes
     isto é vicio de outras liguagem declarar antes as variaveis
     */
     //let mut multiplicador: i32 = 0;
    //let mut resultado: i32 = 0;

    println!("Digite o multiplicador: ");
    let mut input = String::new();
    //vai fazer captura da imagen na tela
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    // vai pegar o input e converter para i32
    let multiplicador: i32 = input
        .trim()
        .parse()
        .expect("Por favor, digite um número inteiro");

    for i in 1..=10 {
        let resultado = multiplicador * i;
        println!("{} x {} = {}", multiplicador, i, resultado);
    }
}
