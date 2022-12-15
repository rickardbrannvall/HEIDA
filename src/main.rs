use concrete::*;
use lib::*;


fn main() {

    let id = "00001";

    println!("Creating Secret key!");
    /*
    Input: ( Dimension, Noise level )
    Output: ( Secret key )
    */
    let sk = new_key(1024, -40);

    save_key(&sk, id);

    let sk_copy = load_key(id);

    print_type_of(&sk);

    println!("{:?}", sk.type_of());

    // println!("Creating Bootstrapping key!");
    // /*
    // Input: ( INPUT key, (OPTIONAL) OUTPUT key )
    // Output: ( Bootstrap key )
    // */
    // let pbs_key = new_PBS_key(&sk, None);
    // println!("{:?}", pbs_key.polynomial_size);

    // println!("Creating Keyswitching key!");
    // /*
    // Input: ( INPUT key, OUTPUT key )
    // Output: ( Keyswitching key ) 
    // */
    // let ks_key = new_KS_key(&sk, &sk);
    // println!("{:?}", ks_key.dimension_after);
}
