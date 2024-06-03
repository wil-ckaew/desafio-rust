use std::io;
use std::io::Write;

#[derive(Debug)]
struct Aluno {
    nome: String,
    nota: f32,
}

fn main() {
    let mut alunos: Vec<Aluno> = Vec::new();

    loop {
        println!("\nO que você deseja fazer?");
        println!("1- Cadastrar aluno");
        println!("2- Alterar aluno");
        println!("3- Excluir aluno");
        println!("4- Listar alunos");
        println!("5- Sair do programa");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();
        let escolha = escolha.trim();

        match escolha {
            "1" => cadastrar_aluno(&mut alunos),
            "2" => alterar_aluno(&mut alunos),
            "3" => excluir_aluno(&mut alunos),
            "4" => listar_alunos(&alunos),
            "5" => {
                println!("Saindo do programa...");
                break;
            },
            _ => println!("Escolha inválida, tente novamente."),
        }
    }
}

fn cadastrar_aluno(alunos: &mut Vec<Aluno>) {
    println!("Iniciando cadastro de aluno");
    
    let mut nome = String::new();
    let mut nota = String::new();

    print!("Digite o nome do aluno: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim().to_string();

    print!("Digite a nota do aluno: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota).unwrap();
    let nota: f32 = match nota.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Nota inválida, por favor insira um número.");
            return;
        }
    };

    let aluno = Aluno { nome, nota };
    alunos.push(aluno);
    println!("Aluno cadastrado com sucesso!");
}

fn alterar_aluno(alunos: &mut Vec<Aluno>) {
    println!("Iniciando alteração de aluno");

    if alunos.is_empty() {
        println!("Nenhum aluno cadastrado.");
        return;
    }

    listar_alunos(&alunos);

    let mut indice = String::new();
    print!("Digite o número do aluno que deseja alterar: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut indice).unwrap();
    let indice: usize = match indice.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Índice inválido.");
            return;
        }
    };

    if indice == 0 || indice > alunos.len() {
        println!("Aluno não encontrado.");
        return;
    }

    let aluno = &mut alunos[indice - 1];

    let mut novo_nome = String::new();
    let mut nova_nota = String::new();

    print!("Digite o novo nome do aluno (deixe vazio para manter o nome atual): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut novo_nome).unwrap();
    let novo_nome = novo_nome.trim();

    if !novo_nome.is_empty() {
        aluno.nome = novo_nome.to_string();
    }

    print!("Digite a nova nota do aluno (deixe vazio para manter a nota atual): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nova_nota).unwrap();
    let nova_nota = nova_nota.trim();

    if !nova_nota.is_empty() {
        aluno.nota = match nova_nota.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Nota inválida, mantendo a nota atual.");
                aluno.nota
            }
        };
    }

    println!("Aluno alterado com sucesso!");
}

fn excluir_aluno(alunos: &mut Vec<Aluno>) {
    println!("Iniciando exclusão de aluno");

    if alunos.is_empty() {
        println!("Nenhum aluno cadastrado.");
        return;
    }

    listar_alunos(&alunos);

    let mut indice = String::new();
    print!("Digite o número do aluno que deseja excluir: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut indice).unwrap();
    let indice: usize = match indice.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Índice inválido.");
            return;
        }
    };

    if indice == 0 || indice > alunos.len() {
        println!("Aluno não encontrado.");
        return;
    }

    alunos.remove(indice - 1);
    println!("Aluno excluído com sucesso!");
}

fn listar_alunos(alunos: &[Aluno]) {
    println!("Listando alunos");

    if alunos.is_empty() {
        println!("Nenhum aluno cadastrado.");
    } else {
        for (i, aluno) in alunos.iter().enumerate() {
            println!("{} - Nome: {}, Nota: {}", i + 1, aluno.nome, aluno.nota);
        }
    }
}
