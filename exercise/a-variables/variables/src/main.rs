const STARTING_MISSIELIES:i32 = 8;
const READY_AMOUNT : i32 = 2;

fn main() {


    let mut missiles = STARTING_MISSIELIES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missles", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles);
}
