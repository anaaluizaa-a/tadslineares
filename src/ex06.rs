pub fn ex06(){
    
    let mut voltar = vec!["google.com", "github.com", "unibh.br"];
    let mut avancar = Vec::new();

    println!("Página atual: {:?}", voltar.last().unwrap());

    if let Some(pagina) = voltar.pop(){
    avancar.push(pagina);
    }

    println!("Após voltar, nova página: {:?}", voltar.last().unwrap());
}
