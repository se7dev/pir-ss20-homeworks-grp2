#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[derive(Debug)]
struct Animal {
    animal_type: String,
    name: String,
    size: f32,
}

#[derive(Debug)]
struct Colors(i32, i32, i32);

impl Animal {
    fn change_animal_type(&mut self, animal_type: String) {
        self.animal_type = animal_type
    }
    fn new(animal_type: String, name: String, size: f32) -> Animal {
        Animal {
            animal_type,
            name,
            size,
        }
    }
}

fn main() {
    let mut duck_donald = Animal {
        animal_type: String::from("Duck"),
        name: String::from("Donald"),
        size: 2.0,
    };

    let mut duck_daisy = Animal::new("Duck".to_string(), "Daisy".to_string(), 1.3);

    duck_donald.change_animal_type("Giraffe".to_string());
    (&mut duck_donald).change_animal_type("Giraffe".to_string());
    //(&duck_donald).change_animal_type("Giraffe".to_string())

    let blue = Colors(0,0,136);
    println!("{:#?}", blue);
    println!("{:#?}", blue.0); 
}
