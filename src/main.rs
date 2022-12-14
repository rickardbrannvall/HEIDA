use concrete::*;
use lib::*;

fn main() {
    println!("Creating keys!");

    // Input: ( Dimmension, Noise level )
    // Output: ( Secret key )
    let sk = new_key(1024, -40);

    println!("{:?}", sk);
}
