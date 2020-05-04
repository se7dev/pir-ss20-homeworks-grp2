#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[derive(Debug)]
struct Animal {
    animal_type: String,
    name: String,
    size: f32,
}

fn main() {
    
    /*Werte vergessen
        let duck_donald = Animal {
            name: String::from("Bert"),
        };
    */

    //mut nicht schreiben
    let duck_donald = Animal {
        animal_type: String::from("Duck"),
        name: String::from("Donald"),
        size: 2.0,
    };
    println!("{:?}", duck_donald);
    println!("{:#?}", duck_donald);

    let mut duck_donald = Animal {
        animal_type: String::from("Duck"),
        name: String::from("Donald"),
        size: 2.0,
    };
    duck_donald.animal_type = String::from("Giraffe");
    println!("{:#?}", duck_donald);


    //struct update syntax
    let mut duck_daisy = Animal {
        name: String::from("Daisy"),
        ..duck_donald
    };
    println!("{:#?}", duck_daisy);

}
