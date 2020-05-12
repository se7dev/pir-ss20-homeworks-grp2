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


// Tests
#[cfg(test)]
mod game_tests{
    use crate::{Hero, Gamefield, Monster};

    #[test]
    fn test_gamefield(){
        // Are gamefields initialized successfully?
        let gfield = Gamefield::create(8);
        assert!(gfield.size == 8);
        assert!(gfield.field[0][0] == '#');
    }

    #[test]
    fn test_hero_monster_structs(){
        let mut gfield = Gamefield::create(8);
        let mut test_hero = Hero::spawn(&mut gfield, String::from("Peter"), 100, 10, (0, 0));

        // Is your hero correctly initialized?
        assert!(test_hero.damage == 10);
        assert!(test_hero.hp == 100);
        assert!(test_hero.position == (0,0));
        assert!(test_hero.name == String::from("Peter"));

        // Does attack work as intended?
        let mut monster_1 = Monster::spawn(&mut gfield, String::from("Monster1"), 100, 10, (7, 8));

        test_hero.attack(&mut monster_1, &mut gfield);

        assert!(monster_1.hp == 90);

        monster_1.attack(&mut test_hero);

        assert!(test_hero.hp == 90);

    }
}


// Tests
#[cfg(test)]
mod game_tests{
    use crate::{Hero, Gamefield, Monster};

    #[test]
    fn test_gamefield(){
        // Are gamefields initialized successfully?
        let gfield = Gamefield::create(8);
        assert!(gfield.size == 8);
        assert!(gfield.field[0][0] == '#');
    }

    #[test]
    fn test_hero_monster_structs(){
        let mut gfield = Gamefield::create(8);
        let mut test_hero = Hero::spawn(&mut gfield, String::from("Peter"), 100, 10, (0, 0));

        // Is your hero correctly initialized?
        assert!(test_hero.damage == 10);
        assert!(test_hero.hp == 100);
        assert!(test_hero.position == (0,0));
        assert!(test_hero.name == String::from("Peter"));

        // Does attack work as intended?
        let mut monster_1 = Monster::spawn(&mut gfield, String::from("Monster1"), 100, 10, (7, 8));

        test_hero.attack(&mut monster_1, &mut gfield);

        assert!(monster_1.hp == 90);

        monster_1.attack(&mut test_hero);

        assert!(test_hero.hp == 90);

    }
}