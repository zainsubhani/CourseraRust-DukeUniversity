fn ownership() {
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[..];
    println!("{:?}", slice);
}
fn modify_vector() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    println!("{:?}", numbers);
}

fn modify_vector2() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let slice = &mut numbers[..];
    slicep[0] = 100;
    println!("{:?}", numbers);
    println!("{:?}", slice);
}

fn main() {
    ownership();
    modify_vector();
}
