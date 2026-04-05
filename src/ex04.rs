pub fn ex04() {
    let v1 = vec![1,2,3,4];
    let v2 = vec![11,12,13,14];
    let mut v3 = v1.clone();
    v3.extend(&v2);
    println!("Vetor mesclado e ordenado (forma fácil): {:?}", v3);

}