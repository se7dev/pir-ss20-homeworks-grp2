use std::vec::Vec;

fn main() {
    let mut vec0 = Vec::new();
    push_on_stack(&mut vec0);
    println!("{:?}", vec0);

    let vec1 = push_on_stack_u(vec0);


    println!("{:?}", vec1);
}

fn push_on_stack(vec: &mut Vec<i32>) {
    vec.extend([40, 50, 60].iter().copied());
}

fn push_on_stack_u(_vec: Vec<i32>) -> Vec<i32> {
    vec![40, 50, 60]
}
