
pub fn ex01() {
    let mut original = vec![1,2,3,4,5];
    let mut invertido = Vec::new();
    while let Some(x) = original.pop() {
        invertido.push(x);
    }
    println!("Vetor Invertido: {:?}", invertido);
}