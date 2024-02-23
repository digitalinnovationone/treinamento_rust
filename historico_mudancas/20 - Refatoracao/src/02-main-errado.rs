fn exemplo_clone(x: i32) {
    let y = x; // Não é necessário usar clone para tipos Copy
    println!("x: {}, y: {}", x, y);
}

fn main() {
    exemplo_clone(5);
}
