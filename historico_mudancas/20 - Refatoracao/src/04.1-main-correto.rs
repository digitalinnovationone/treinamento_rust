fn exemplo_loop() {
    let vetor = vec![1, 2, 3];
    for valor in &vetor {
        println!("{}", valor);
    }
}

fn main() {
    exemplo_loop();
}
