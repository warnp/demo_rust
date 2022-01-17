use std::error::Error;
use std::fmt::{Display, Formatter, write};

trait Actor {
    fn attack(&self, target: &mut dyn Actor) -> Result<i32, LifeError>;
    fn get_name(&self) -> &str;
    fn get_damage(&mut self, damage_amount: i32) -> Result<i32, LifeError>;
}

#[derive(Debug, Clone)]
struct Item {
    pub name: String,
    pub damage: i32,
}

#[derive(Debug, Clone)]
struct Perso<'a> {
    pub name: String,
    pub life: i32,
    pub inventory: Vec<Item>,
    pub equipped_item: Option<&'a Item>,
}

impl<'a> Actor for Perso<'a> {
    fn attack(&self, target: &mut dyn Actor) -> Result<i32, LifeError> {
        println!("{} attack {} with his/her {}", self.name, target.get_name(), self.equipped_item.map_or_else(|| "pas d'arme", |i| &i.name));
        //Gestion des optional
        if let Some(damage) = self.equipped_item {
            return target.get_damage(damage.damage);
        }
        Ok(0)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_damage(&mut self, damage_amount: i32) -> Result<i32, LifeError> {
        Ok(self.life)
    }
}


#[derive(Debug, Clone)]
struct BadGuy {
    pub name: String,
    pub life: i32,
    pub equipped_item: Item,
}

impl Actor for BadGuy {
    fn attack(&self, target: &mut dyn Actor) -> Result<i32, LifeError> {
        println!("{} attack {} with his/her {}", self.name, target.get_name(), self.equipped_item.name);
        Ok(0)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_damage(&mut self, damage_amount: i32) -> Result<i32, LifeError> {
        self.life = self.life - damage_amount;
        if self.life > 0 {
            return Ok(self.life);
        }
        Err(LifeError {})
    }
}

#[derive(Debug)]
struct LifeError {}

impl Display for LifeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Warrior is dead")
    }
}

impl Error for LifeError {}

fn main() {
    let sword = Item {
        name: "Big sword".to_string(),
        damage: 5,
    };

    let axe = Item {
        name: "Big Axe".to_string(),
        damage: 6,
    };

    let mut our_hero = Perso {
        name: "Toto".to_string(),
        life: 100,
        inventory: vec![sword],
        equipped_item: None,
    };

    let mut bad_guy = BadGuy {
        name: "Biff".to_string(),
        life: 100,
        equipped_item: axe,
    };

    println!("Hello, {our_hero:?}");

    println!("{} equip a weapon", our_hero.name);
    //We have to clone our_hero just right here to avoid potential mutable borrowing more than one
    // time error, it's possible to use Rc or any other smartpointer option to do the same
    let hero_copy = our_hero.clone();
    our_hero.equipped_item = hero_copy.inventory.get(0);

    loop {
        let our_hero_attack_result = our_hero.attack(&mut bad_guy);
        let bad_guy_attack_result = bad_guy.attack(&mut our_hero);

        match our_hero_attack_result {
            Ok(r) => println!("bad guy have {:?} PVs remaining", r),
            Err(r) => {
                println!("bad guy is dead");
                break
            }
        }
    }
}
