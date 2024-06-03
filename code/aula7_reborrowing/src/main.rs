fn main() {
    //memória stack (variáveis do tipo copy no rust by copy)
    let mut x: i32 = 4;
    println!("[Original] - Valor de x após as modificações: {} - referência: {:p}", x, &x);

    imprime_valor(&mut x); // Passando uma referência mutável para x
    
    println!("[Original] - Valor de x após as modificações: {} - referência: {:p} ", x, &x);

    imprime_valor(&mut x); //Passando uma referência mutável para x

    println!("[Original] - Valor de x após as modificações: {} - referência: {:p}", x, &x);
}

fn imprime_valor(valor: &mut i32){
    //valor += 1; //não pode porque tenho imutabilidade nas referencias
    *valor += 1; //Modificando o valor referenciado por valor utilizando um reborrowing
    // O compilador vode mover a variável temporariamente para uma localização diferente na memória durante a referência mutável.
    // O objetivo é evitar possíveis problemas de aliasing e garantir a segurança das referência mutáveis
    println!("[reborrowing] - Valor referenciado por valor: {} - referência: {:p}", valor, &valor);
}
