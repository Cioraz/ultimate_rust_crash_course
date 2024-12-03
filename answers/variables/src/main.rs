const READY_AMOUNT: i32 = 2;
const STARTING_MISSILES: i32 = 8;

fn main() {
    let new_variable: i32 = 2100;
    READY_AMOUNT = 1;
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
