#[derive(Debug)]
struct Item {
    name: String,
    damage: i32
}

#[derive(Debug)]
struct Perso {
    name: String,
    life: i32,
    inventory: Vec<Item>
}

fn main() {
    let sword = Item {
        name: "Big sword".to_string(),
        damage: 5
    };

    let our_hero = Perso {
        name: "Toto".to_string(),
        life: 100,
        inventory: vec![sword]
    };

    println!("Hello, {our_hero:?}");
}
