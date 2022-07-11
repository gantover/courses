const STARTING_MISSILES: i32 = 8;
const READY_MISSILES: i32 = 2;
fn main() {
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_MISSILES);
    println!("Firing {} of my {} ready missiles", ready, missiles);
    println!("I have {} missiles left", missiles - ready);
}
