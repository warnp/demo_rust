trait Actor {
    fn attack(&self, target: &dyn Actor) -> bool;
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
struct Item {
    pub name: String,
    pub damage: i32
}

#[derive(Debug)]
struct Perso<'a> {
    pub name: String,
    pub life: i32,
    pub inventory: Vec<Item>,
    pub equipped_item: Option<&'a Item>
}

impl<'a> Actor for Perso<'a > {
    fn attack(&self, target: & dyn Actor) -> bool {
        println!("{} attack {} with his/her {}", self.name, target.get_name(), self.equipped_item.map_or_else(|| "pas d'arme", |i| &i.name));
        true
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}



#[derive(Debug)]
struct BadGuy {
    pub name: String,
    pub life: i32,
    pub equipped_item: Item
}

impl Actor for BadGuy {
    fn attack(&self, target: & dyn Actor) -> bool {
        println!("{} attack {} with his/her {}", self.name, target.get_name(), self.equipped_item.name);
        true
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let sword = Item {
        name: "Big sword".to_string(),
        damage: 5
    };

    let axe = Item {
        name: "Big Axe".to_string(),
        damage: 6
    };

    let mut our_hero = Perso {
        name: "Toto".to_string(),
        life: 100,
        inventory: vec![sword],
        equipped_item: None
    };

    let bad_guy = BadGuy {
        name: "Biff".to_string(),
        life: 100,
        equipped_item: axe,
    };

    println!("Hello, {our_hero:?}");
    println!("Attack : {}", our_hero.attack(&bad_guy));
    println!("Attack : {}", bad_guy.attack(&our_hero));
    println!("{} equip a weapon", our_hero.name);
    our_hero.equipped_item = our_hero.inventory.get(0);
}
