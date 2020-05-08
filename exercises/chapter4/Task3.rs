fn main() {
    let mut a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    change_slice(&mut a[0..1], 4);
    println!("{:?}", a);
    change_slice(&mut a[2..3], 6);
    println!("{:?}", a)
}

fn change_slice(slice: &mut [i32], value: i32) {
    slice[0] = value;
}