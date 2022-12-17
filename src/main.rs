// use concrete::*;
use lib::*;


fn main() {

    let id = "00001";

    // println!("Creating Secret key!");

    // let sk = new_key(1024, -40);

    // sk.save_key(id);

    println!("Loading Secret key!");
    let sk = load_sk_key(id);

    print_type_of(&sk);
    println!("{:?}", sk.type_of());

    // println!("Creating Bootstrapping key!");
    // // let pbs_key = new_PBS_key(&sk, None);
    // let rlwe_key = sk.to_rlwe_secret_key(sk.dimension).unwrap(); 
    // let pbs_key = LWEBSK::new(&sk, &rlwe_key, 6, 6);

    // println!("Saving Bootstrapping key!");
    // pbs_key.save("keys/BSK00001");
    // pbs_key.save_key(id);

    println!("Loading Bootstrapping key!");
    let pbs_key = load_bsk_key(id);
    println!("{:?}", pbs_key.polynomial_size);

    // println!("Creating Keyswitching key!");
    // let ks_key = new_KS_key(&sk, &sk);
    // ks_key.save_key(id);
    println!("Loading Keyswitching key!");
    let ks_key = load_ksk_key(id);
    println!("{:?}", ks_key.dimension_after);
}
