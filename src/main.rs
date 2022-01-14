#[derive(Debug)]
struct Perso {
    name: String,
    life: i32,
}

fn main() {

    let our_hero = Perso {
        name: "Toto".to_string(),
        life: 100
    };

    println!("Hello, {our_hero:?}");
}
