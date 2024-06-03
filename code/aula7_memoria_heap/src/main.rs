/*
fn main() {
    //variáveis na memória heap

    let s1 = String::from("Olá"); // s1 possui a propriedade da String

    // Isso causa um erro, porque s1 não é mais válido após a transferência
    println!("Antes da transferência:");
    print_string(s1); 
    // print_string(s1); //o s1 não é mais valido e nem o s da função print_string pois s1 não é mais dono variável
    //println!("{}", s);
}

fn print_string(s: String) {
    println!("Valor da String: {} - referência: {:p}", s, &s);
}
*/
/*
fn main() {
    //variáveis na memória heap

    let s1 = String::from("Olá"); // s1 possui a propriedade da String

    // Isso causa um erro, porque s1 não é mais válido após a transferência
    println!("Antes da transferência:");
    print_string(&s1); 
    print_string(&s1); //agora o codigo passa a compilar pois estou apontando para endereçamento de memória
}

fn print_string(s: &String) {
    println!("Valor da String: {} - referência: {:p}", s, &s);
}
*/

/*
fn main() {
    let mut x = 5;
   // println!("{} - {:p}", x, &x);

    manda_referencia(&mut x);

   // println!("{} - {:p}", x, &x);
}
// aqui passei via copy e ela é mutavel
fn manda_referencia(x: &mut i32){
    *x += 1;
    //println!("{} - {:p}", x, &x)
    println!("{}", x)
}
*/
/*
fn main() {
    let x = 5;
    println!("{} - {:p}", x, &x);

    manda_referencia(x);

    println!("{} - {:p}", x, &x);
}
// aqui passei via copy e ela é mutavel
fn manda_referencia(mut x: i32){
    x += 1;
    println!("{} - {:p}", x, &x)
}
*/
fn main() {
    let x = 5;
    
    manda_via_copia(x);

    println!("{}", x);
}

fn manda_via_copia(x: i32) -> i32{
    x +1
}