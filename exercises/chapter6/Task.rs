//TASK:
// In chapter 6 of the Rust book(link to) you learned that the Option<T> type is actually
// an enum. Your task is to define your own: make an enum VerboseOption<T> which contains
// either the generic type T, or an explanation of some sort as to why we failed.
// Note: you cannot name your variants Some and None, because they are included in the
// rust prelude (auto imported language features).
use std::env;

#[derive(Debug)]
enum VerboseOption<T> {
    Present(T),
    Absent,
}
#[derive(Debug)]
enum NaturalNumberKind {
    DivisThree(u8),
    DivisFive(u8),
    DivisBoth(u8),
    DivisNeither(u8),
}

//TASK: Next, write a binary that will take two u8 from the command line and attempt run them through the following calculation:


fn calculate_result(a: u8, b: u8) -> VerboseOption<u8> {
// Guard this calculation against under and overflows, returning your VerboseOption<u8>
    //let res = ((a * b) + 3 * a - (2 * b)) - a - b;
    let mut res = a.checked_mul(b);
    if !res.is_some(){
        return VerboseOption::Absent
    }
    let mut x = a.checked_mul(3);
    if !res.is_some(){
        return VerboseOption::Absent
    }
    res = res.unwrap().checked_add(x.unwrap());
    if !res.is_some(){
        return VerboseOption::Absent
    }
    x = b.checked_mul(2);
    if !res.is_some(){
        return VerboseOption::Absent
    }
    res = res.unwrap().checked_sub(x.unwrap());
    if !res.is_some(){
        return VerboseOption::Absent
    }
    res = res.unwrap().checked_sub(a);
    if !res.is_some(){
        return VerboseOption::Absent
    }
    res = res.unwrap().checked_sub(b);
    //VerboseNone("To be implemented".to_string())
    let result = VerboseOption::Present(res.unwrap());
    result
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut a: u8 = 0;
    let mut b: u8 = 0;
    match args.len() {
        3 => {
            let arg1 = &args[1];
            let arg2 = &args[2];
            a = match arg1.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("error: first argument not an integer");
                    return;
                }
            };
            b = match arg2.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    return;
                }
            };
        }
        _ => {
            println!("Not the right number of args")
        }
    }
    println!("a: {} b: {}", a, b);
    let result = calculate_result(a, b);
    println!("Result is {:?}", result)
}