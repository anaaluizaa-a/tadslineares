pub fn ex05(){
    let mut pilha: Vec<f64> = Vec::new();
    let entrada = "3 4 + 2 *";

    for item in entrada.split_whitespace() {
        match item {
            "+" => {
                let b = pilha.pop().unwrap(); // Tira o 4
                let a = pilha.pop().unwrap(); // Tira o 3
                pilha.push(a + b);
            }
            "*" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a*b);
            }
            numero => {
                // Se for número, transformamos o texto em valor real
                if let Ok(valor) = numero.parse::<f64>() {
                    pilha.push(valor);
                }
            }
        }
    }
    println!("O resultado é {:?}", pilha);
}
