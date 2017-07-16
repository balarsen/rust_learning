

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
        println!("{} is eating.", self.name);

        // thread::sleep_ms(1000); This is depricated by a compiler warning
        std::thread::sleep(std::time::Duration::from_millis(1000));

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
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    for p in &philosophers {
        p.eat();
    }
}
