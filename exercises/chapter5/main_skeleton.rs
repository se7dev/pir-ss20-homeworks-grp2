// Define a Hero Struct
#[derive(...)]
struct Hero {

}
// Let Hero spawn, attack and move
impl Hero {
    // fn spawn(...) -> ... {
    //
    // }
    // fn attack(...) {
    //     ...
    // }
    // fn move_hero(...) {
    //     ...
    // }
}

// Define a Monster Struct
#[derive(...)]
struct Monster {

}
// Let Monster spawn, attack and move
impl Monster {
    // fn spawn(...) -> ... {
    //     ...
    // }
    //
    // fn attack(...) {
    //     ...
    // }
    // fn move_monster(...) {
    //    ...
    // }
    // // Make monster cloneable
    // fn clone(...) {
    //     ...
    // }
}

#[derive(..., ...)]
struct Gamefield {

}

impl Gamefield {
    // fn create(...) -> ... {
    //    ...
    // }
    // fn display(...) {
    //     ...
    // }
}


fn main() {
    // Create Gamefield
    // let mut gamefield = ...;
    // Spawn a hero
    // let mut hero = ...

    // Spawn a Monster
    // let mut monster = ...

    // Let them fight, and clone a monster
    // hero.attack(...);
    // monster_1.attack(...);
    // Move hero and mosnter
    // hero.move_hero(...);
    // monster_1.move_monster(...);
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