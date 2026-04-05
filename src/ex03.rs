pub fn ex03(){
    let v= vec![1,2,3,4,5,6,7,8,9,10];
    let mut apenas_impares = Vec::new();
    for n in &v{
        if *n %2 !=0 {
            apenas_impares.push(*n);
        }
    }
    println!("Numeros impares são: {:?}", apenas_impares);
    }
