pub fn ex07(){

    let mut desfazer = vec![1,2,3];
    let mut refazer = Vec::new();
    println!("Histórico: {:?}", desfazer);

    if let Some(acao)= desfazer.pop(){
        refazer.push(acao);
    }
    println!("Historico atual: {:?}", desfazer);
    println!("E na pilha refazer: {:?}", refazer);

}