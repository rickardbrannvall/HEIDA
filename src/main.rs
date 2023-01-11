// use concrete::*;
use lib::*;

fn main() {
    
    let id = "00001";

    // println!("Creating Secret key!");
    // let sk = new_key(1024, -40);
    // sk.save_key(id);

    println!("Loading Secret key!");
    let sk = load_sk(id);
    // print_type_of(&sk);
    // println!("{:?}", sk.type_of());

    // let 



    // // println!("Creating Bootstrapping key!");
    // // let bsk = new_bsk(&sk, None);

    // // println!("Saving Bootstrapping key!");
    // // bsk.save_key(id);

    // println!("Loading Bootstrapping key!");
    // let bsk = load_bsk(id);
    // println!("{:?}", bsk.polynomial_size);



    // // println!("Creating Keyswitching key!");
    // // let ksk = new_ksk(&sk, &sk);

    // // println!("Saving Keyswitching key!");
    // // ksk.save_key(id);

    // println!("Loading Keyswitching key!");
    // let ksk = load_ksk(id);
    // println!("{:?}", ksk.dimension_after);
}
