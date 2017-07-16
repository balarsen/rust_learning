struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
    fn eat(&self) {
        println!("{} is done eating.", self.name);
    }
}
// without this above we would have to do
// fn main() {
//     let p1 = Philosopher { name: "Judith Butler".to_string() };
//     let p2 = Philosopher { name: "Gilles Deleuze".to_string() };
//     let p3 = Philosopher { name: "Karl Marx".to_string() };
//     let p4 = Philosopher { name: "Emma Goldman".to_string() };
//     let p5 = Philosopher { name: "Michel Foucault".to_string() };
// }

fn main() {
    let p1 = Philosopher::new("Judith Butler");
    let p2 = Philosopher::new("Gilles Deleuze");
    let p3 = Philosopher::new("Karl Marx");
    let p4 = Philosopher::new("Emma Goldman");
    let p5 = Philosopher::new("Michel Foucault");
}
