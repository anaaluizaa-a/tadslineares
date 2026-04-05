
    struct StackMin{
        principal: Vec<i32>,
        minimo: Vec<i32>,
    }

    impl StackMin{
        fn new() -> Self {
            StackMin {
                principal: Vec::new(),
                minimo: Vec::new(),
            }
        }

        fn push(&mut self, valor: i32){
            self.principal.push(valor);
        
        if self.minimo.is_empty(){
            self.minimo.push(valor);
        }
    }
    }

    
pub fn ex09(){
    let mut pilha = StackMin::new();
    pilha.push(10);
    pilha.push(30);
    pilha.push(45);
    
    print!("O menor valor é:{:?}", pilha.minimo.last());
}