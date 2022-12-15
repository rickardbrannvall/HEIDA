use concrete::*;
use lib::*;


fn main() {

    let id = "00001_in";

    println!("Creating Secret key!");

    let sk = new_key(1024, -40);

    sk.save_key(id);

    let sk_copy = load_key(id);

    print_type_of(&sk);
    println!("{:?}", sk.type_of());

    // println!("Creating Bootstrapping key!");
    // let pbs_key = new_PBS_key(&sk, None);
    // println!("{:?}", pbs_key.polynomial_size);

    // println!("Creating Keyswitching key!");
    // let ks_key = new_KS_key(&sk, &sk);
    // println!("{:?}", ks_key.dimension_after);
}
