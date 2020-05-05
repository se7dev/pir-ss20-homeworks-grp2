// Define a Hero Struct
#[derive(...)]
struct Hero {

}
// Let Hero spawn, attack and move
impl Hero {
    // fn spawn(gamefield: &mut Gamefield, name: String, hp: i64, damage: i64, mut position: (i32, i32)) -> Hero {
    // ...your Code here
    // Hero {
    //  name,
    //  hp,
    //  damage,
    //  position,
    // }
    // }
    // fn attack(&self, enemy: &mut Monster, gamefield: &mut Gamefield) {
    //     ...
    // }
    // fn move_hero(&mut self, gamefield: &mut Gamefield, direction: &str) {
    //     ...
    // }
}

// Define a Monster Struct
#[derive(...)]
struct Monster {

}
// Let Monster spawn, attack and move
impl Monster {
    // fn spawn(gamefield: &mut Gamefield, name: String, hp: i64, damage: i64, mut position: (i32, i32)) -> Monster {
    // ...your Code here
    // Hero {
    //  name,
    //  hp,
    //  damage,
    //  position,
    // }
    // }
    //
    // fn attack(&self, hero: &mut Hero) {
    //     ...
    // }
    // fn move_monster(&mut self, gamefield: &mut Gamefield, direction: &str) {
    //     More code here...
    // gamefield.field[old_pos.0 as usize][old_pos.1 as usize] = '#';
    // gamefield.field[self.position.0 as usize][self.position.1 as usize] = 'M';
    // Gamefield::display(gamefield.clone());
    //
    // }
    // // Make monster cloneable
    // fn clone(to_clone: &Monster, gamefield: &mut Gamefield) {
    //     ...
    // }
}

#[derive(..., ...)]
struct Gamefield {

}

impl Gamefield {
    // fn create(size: usize) -> Gamefield {
    //    ...
    // }
    // fn display(gamefield: Gamefield) {
    //     ...
    // }
}


fn main() {
    // Create Gamefield
    let mut gamefield = Gamefield::create(8);
    // Spawn a hero
    let mut hero_peter = Hero::spawn(&mut gamefield, String::from("Peter"), 100, 10, (0, 0));


    // // Spawn a Monster
    let mut monster_1 = Monster::spawn(&mut gamefield, String::from("Monster1"), 54, 10, (7, 8));

    // Let them fight, and clone a monster
    hero_peter.attack(&mut monster_1, &mut gamefield);
    monster_1.attack(&mut hero_peter);
    //
    hero_peter.move_hero(&mut gamefield, "up");
}
