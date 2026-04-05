pub fn ex08(){
    let expressao = "{[()]}";
    let mut pilha = Vec::new();
    let mut balanceado = true;

    for char in expressao.chars() {
        match char {
            '('| '[' |'{' => pilha.push(char),

            ')' => {
                if pilha.pop()!= Some('('){
                    balanceado = false;
                    break
                }
            }
            ']' => {
                if pilha.pop()!= Some('['){
                    balanceado = false;
                    break
                }
            }
            '}' => {
                if pilha.pop()!= Some('{'){
                    balanceado = false;
                    break
                }
            }
    _=> {}

            }
        }

        if balanceado && pilha.is_empty(){
            println!("A expressão {:?} está balanceada! ", expressao);
    } else {
        println!("A expressão {:?} está desbalanceada! ", expressao);
    }
        
}

