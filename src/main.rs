use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::fmt::{Display, format, Formatter, write};
use std::sync::{Arc, Mutex};
use futures::executor::block_on;
use futures::join;

trait Actor {
    fn attack(&self, target: Arc<Mutex< dyn Actor>>) -> Result<i32, LifeError>;
    fn get_name(&self) -> &str;
    fn get_damage(&mut self, damage_amount: i32) -> Result<i32, LifeError>;
}

#[derive(Debug, Clone)]
struct Item {
    pub name: String,
    pub damage: i32,
}

#[derive(Debug, Clone)]
struct Perso {
    pub name: String,
    pub life: i32,
    pub inventory: Vec<Item>,
    pub equipped_item: Option<Arc<Item>>,
}

impl Actor for Perso {
    fn attack(&self, target: Arc<Mutex<dyn Actor>>) -> Result<i32, LifeError> {
        println!("{} attack {} with his/her {}", self.name, target.lock().unwrap().get_name(), self.equipped_item.clone().map_or_else(|| "pas d'arme".to_string(), |i| i.name.clone()));
        //Gestion des optional
        if let Some(damage) = self.equipped_item.clone() {
            return target.lock().unwrap().get_damage(damage.damage);
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
    fn attack(&self, target: Arc<Mutex< dyn Actor>>) -> Result<i32, LifeError> {
        println!("{} attack {} with his/her {}", self.name, target.lock().unwrap().get_name(), self.equipped_item.name);
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

#[tokio::main]
async fn main() {
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
    let inventory = hero_copy.inventory.get(0).unwrap().clone();
    our_hero.equipped_item = Some(Arc::new(inventory));

    for el in 0..100 {
        let hero_arc = Arc::new(Mutex::new(our_hero.clone()));
        hero_arc.lock().unwrap().name = format!("{}_{}", hero_arc.lock().unwrap().name, el);
        let bad_guy_arc = Arc::new(Mutex::new(bad_guy.clone()));
        bad_guy_arc.lock().unwrap().name = format!("{}_{}", bad_guy_arc.lock().unwrap().name, el);

        let spawn = tokio::spawn(async move {
            combat_loop(hero_arc, bad_guy_arc).await;
        });
    }
}

async fn combat_loop( our_hero: Arc<Mutex<Perso>>, bad_guy: Arc<Mutex<BadGuy>>) -> () {

    loop {
        let our_hero_attack_result = our_hero.lock().unwrap().attack(bad_guy.clone());
        let bad_guy_attack_result = bad_guy.lock().unwrap().attack(our_hero.clone());

        match our_hero_attack_result {
            Ok(r) => println!("bad guy have {:?} PVs remaining", r),
            Err(r) => {
                println!("bad guy is dead");
                break
            }
        }

    }
    ()
}
