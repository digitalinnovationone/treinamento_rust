// trait Display {
//     fn display(&self) -> String;
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Display for Point {
//     fn display(&self) -> String {
//         format!("Point(x: {}, y: {})", self.x, self.y)
//     }
// }

// struct Circle {
//     x: i32,
//     y: i32
// }

// impl Display for Circle {
//     fn display(&self) -> String {
//         format!("Circle(x: {}, y: {})", self.x, self.y)
//     }
// }

// // fn print_display(item: &dyn Display) {
// fn print_display(item: &impl Display) {
//     println!("{}", item.display());
// }

// fn main() {
//     let point = Point { x: 5, y: 10 };
//     let circle = Circle { x: 15, y: 25 };

//     print_display(&point);
//     print_display(&circle);
// }





// // ========= Polimorfismo ============
// trait Display {
//     fn display(&self) -> String;
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Display for Point {
//     fn display(&self) -> String {
//         format!("Point(x: {}, y: {})", self.x, self.y)
//     }
// }

// struct Circle {
//     x: i32,
//     y: i32,
//     radius: i32,
// }

// impl Display for Circle {
//     fn display(&self) -> String {
//         format!("Circle(x: {}, y: {}, radius: {})", self.x, self.y, self.radius)
//     }
// }

// fn print_display(item: &impl Display) {
//     println!("{}", item.display());
// }

// fn main() {
//     let point = Point { x: 5, y: 10 };
//     let circle = Circle { x: 15, y: 25, radius: 5 };

//     print_display(&point);
//     print_display(&circle);
// }







// trait Logger {
//     fn log(&self, message: &str);
// }

// struct ConsoleLogger;

// impl Logger for ConsoleLogger {
//     fn log(&self, message: &str) {
//         println!("Console Logger: {}", message);
//     }
// }

// struct FileLogger;

// impl Logger for FileLogger {
//     fn log(&self, message: &str) {
//         // escrever o erro.log no diretório
//         println!("File Logger: {}", message);
//     }
// }

// fn log_message(logger: &dyn Logger, message: &str) {
//     logger.log(message);
// }

// fn main() {
//     let console_logger = ConsoleLogger;
//     let file_logger = FileLogger;

//     log_message(&console_logger, "Hello, world!");
//     log_message(&file_logger, "Logging to a file now.");
// }








trait Display {
    fn display(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn display(&self) -> String {
        format!("Point(x: {}, y: {})", self.x, self.y)
    }
}

struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}

impl Display for Circle {
    fn display(&self) -> String {
        format!("Circle(x: {}, y: {}, radius: {})", self.x, self.y, self.radius)
    }
}


/* 

Use &impl Trait para casos em que o desempenho é crítico 
e/ou quando você está trabalhando com funções que serão aplicadas a 
um único tipo concreto por chamada, aproveitando o despacho estático 
para eficiência.

Use &impl Trait para casos mais simples e leves onde não haverá polimorfismo:
*/

fn print_display_single_with_impl(item: &impl Display) { // &impl Display(Trait) = "Sintaxe de Traits Concretos" (Syntactic Sugar for Concrete Trait Bounds)
    println!("{}", item.display());
}

/*
&dyn Display usa o que é conhecido como "Trait Objects". 
Isso é mais apropriado para casos em que você precisa armazenar 
múltiplos tipos que implementam o mesmo trait em uma única coleção 
ou quando você precisa de flexibilidade para lidar com vários tipos 
em tempo de execução.

Use &dyn Trait quando você precisar de flexibilidade máxima 
e estiver disposto a aceitar o custo de desempenho do despacho dinâmico.

Use &dyn Trait para tipos mais complexos e quando for fazer polimorfismo

*/

// Usando &dyn Display ( se estivesse montando um Pattern Observer por exemplo )
fn print_display_multiple_with_dyn(items: &[&dyn Display]) { // &dyn Display(Trait) = "Trait Objects"
    for item in items {
        println!("{}", item.display());
    }
}

fn main() {
    let point = Point { x: 5, y: 10 };
    let circle = Circle { x: 15, y: 25, radius: 5 };

     // Chama com &impl Display
     print_display_single_with_impl(&point);
     print_display_single_with_impl(&circle);
 
     // Chama com &dyn Display
     let shapes: [&dyn Display; 2] = [&point, &circle];
     print_display_multiple_with_dyn(&shapes);
}

