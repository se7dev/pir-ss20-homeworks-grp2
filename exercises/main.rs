// Define a Hero Struct
#[derive(Debug)]
struct Hero {
    name: String,
    hp: i64,
    damage: i64,
    position: (i32, i32),
}
// Let Hero spawn, attack and move
impl Hero {
    fn spawn(gamefield: &mut Gamefield, name: String, hp: i64, damage: i64, mut position: (i32, i32)) -> Hero {
        if position.0 >= gamefield.size {
            position.0 = gamefield.size - 1;
        } else if position.1 >= gamefield.size {
            position.1 = gamefield.size - 1;
        }
        gamefield.field[position.0 as usize][position.1 as usize] = 'H';
        Gamefield::display(gamefield.to_owned());
        println!("A new Hero appeared: {}, he is that ({}) healthy, and that ({}) brawny", name, hp, damage);
        Hero {
            name,
            hp,
            damage,
            position,
        }
    }
    fn attack(&self, enemy: &mut Monster, gamefield: &mut Gamefield) {
        enemy.hp = enemy.hp - self.damage;
        println!("{} attacked {} and did {} damage.", self.name, enemy.name, self.damage);
        if enemy.hp < 50 {
            Monster::clone(enemy, gamefield)
        }
    }
    fn move_hero(&mut self, gamefield: &mut Gamefield, direction: &str) {
        let old_pos = self.position;
        if direction == "up" && self.position.0 < 8 {
            self.position.0 = self.position.0 + 1
        } else if direction == "down" && self.position.0 > 0 {
            self.position.0 = self.position.0 - 1
        } else if direction == "left" && self.position.1 > 0 {
            self.position.0 = self.position.1 + 1
        } else if direction == "right" && self.position.1 < 8 {
            self.position.0 = self.position.1 - 1
        }
        gamefield.field[old_pos.0 as usize][old_pos.1 as usize] = '#';
        gamefield.field[self.position.0 as usize][self.position.1 as usize] = 'H';
        Gamefield::display(gamefield.clone());
    }
}

//Let Hero defend

// Define a Monster Struct
#[derive(Debug)]
struct Monster {
    name: String,
    hp: i64,
    damage: i64,
    position: (i32, i32),
}
// Let Monster spawn, attack and move
impl Monster {
    fn spawn(gamefield: &mut Gamefield, name: String, hp: i64, damage: i64, mut position: (i32, i32)) -> Monster {
        if position.0 >= gamefield.size {
            position.0 = gamefield.size - 1;
        } else if position.1 >= gamefield.size {
            position.1 = gamefield.size - 1;
        }
        gamefield.field[position.0 as usize][position.1 as usize] = 'M';
        Gamefield::display(gamefield.to_owned());
        println!("A new Monster appeared: {}, it is that ({}) healthy, and that ({}) brawny", name, hp, damage);
        Monster {
            name,
            hp,
            damage,
            position,
        }
    }
    // Let monster attack and defend
    fn attack(&self, hero: &mut Hero) {
        hero.hp = hero.hp - self.damage;
        println!("{} attacked {} and did {} damage.", self.name, hero.name, self.damage);
    }
    fn move_monster(&mut self, gamefield: &mut Gamefield, direction: &str) {
        let old_pos = self.position;
        if direction == "up" && self.position.0 < 8 {
            self.position.0 = self.position.0 + 1
        } else if direction == "down" && self.position.0 > 0 {
            self.position.0 = self.position.0 - 1
        } else if direction == "left" && self.position.1 > 0 {
            self.position.0 = self.position.1 + 1
        } else if direction == "right" && self.position.1 < 8 {
            self.position.0 = self.position.1 - 1
        }
        gamefield.field[old_pos.0 as usize][old_pos.1 as usize] = '#';
        gamefield.field[self.position.0 as usize][self.position.1 as usize] = 'M';
        Gamefield::display(gamefield.clone());
    }
    // Make monster cloneable
    fn clone(to_clone: &Monster, gamefield: &mut Gamefield) {
        let mut monster = Monster {
            name: String::from("MonsterClone"),
            position: (to_clone.position.0 - 1, to_clone.position.1 - 1),
            ..*to_clone
        };
        Monster::spawn(gamefield, monster.name, monster.hp, monster.damage, monster.position);
    }
}

#[derive(Debug, Clone)]
struct Gamefield {
    field: Vec<Vec<char>>,
    size: i32,
}

impl Gamefield {
    fn create(size: usize) -> Gamefield {
        Gamefield {
            field: vec![vec!['#'; size]; size],
            size: size as i32,
        }
    }
    fn display(gamefield: Gamefield) {
        gamefield.field.into_iter().for_each(|it| {
            println!("{:?}", it);
        });
    }
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
