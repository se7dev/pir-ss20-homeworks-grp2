fn main() {
    let mut s = String::from("Hello World");
    let len = s.chars().count();
    change_slice(&mut s[..],len );
}

fn change_slice(slice: &mut str, i: usize) {
    if i == 0 {
        println!("{}", slice);
        return;
    }
    println!("{}", slice);
    change_slice(&mut slice[..i], i - 1)
}