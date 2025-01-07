fn loop_panic() {
    for i in 0..10 {
        if i == 5 {
            panic!("Panic at {}", i);
        }
    }
}
fn main() {
    loop_panic();
}
