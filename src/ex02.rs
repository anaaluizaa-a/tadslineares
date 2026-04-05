use std::collections::HashMap;
pub fn ex02() {
    let v = vec!['c','h','a','r','i','n','t'];
    let mut cont = HashMap::new();
    for letra in &v{
        let contagem_atual=cont.entry(*letra).or_insert(0);
        *contagem_atual+=1;
    }
        println!("Resultado da contagem: {:?}", cont);
    }
