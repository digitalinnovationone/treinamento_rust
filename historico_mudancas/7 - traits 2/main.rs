// trait Speak {
//     fn speak(&self);
// }

// struct Dog;

// impl Speak for Dog {
//     fn speak(&self) {
//         println!("Woof!");
//     }
// }

// struct Cat;

// impl Speak for Cat {
//     fn speak(&self) {
//         println!("Meow!");
//     }
// }

// fn main() {
//     let dog = Dog;
//     let cat = Cat;

//     dog.speak();
//     cat.speak();
// }







// trait Speak {
//     fn speak(&self);
// }

// struct Dog;

// impl Speak for Dog {
//     fn speak (&self) {
//         println!("Woof!");
//     }
// }

// struct Cat;

// impl Speak for Cat {
//     fn speak(&self) {
//         println!("Meow!");
//     }
// }

// fn speak_now(animal: &dyn Speak){
//     animal.speak();
// }

// fn main() {
//     let dog = Dog;
//     let cat = Cat;

//     speak_now(&dog);
//     speak_now(&cat);
// }








trait Display {
    fn display(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn mostrar(&self) -> String {
        format!("Point(x: {}, y: {})", self.x, self.y)
    }
}

impl Display for Point {
    fn display(&self) -> String {
        format!("Point(x: {}, y: {})", self.x, self.y)
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl Display for Point2 {
    fn display(&self) -> String {
        format!("Point(x: {}, y: {})", self.x, self.y)
    }
}

fn print_display(item: &impl Display) {
    println!("{}", item.display());
}

fn main() {
    let point = Point { x: 5, y: 10 };
    point.mostrar();
    point.display();

    let point2 = Point2 { x: 5, y: 10 };

    point2.display();

    print_display(&point);
    print_display(&point2);
}



/*

Use &dyn Trait quando você precisar de flexibilidade máxima 
e estiver disposto a aceitar o custo de desempenho do despacho dinâmico.

Use &dyn Trait para tipos mais complexos e quando for fazer polimorfismo

*/

/* 

Use &impl Trait para casos em que o desempenho é crítico 
e/ou quando você está trabalhando com funções que serão aplicadas a 
um único tipo concreto por chamada, aproveitando o despacho estático 
para eficiência.

Use &impl Trait para casos mais simples e leves onde não haverá polimorfismo:
*/