use lib::*;

fn main() {

    let id = "00001";

    println!("Creating Secret key!");
    // let sk = new_key(1024, -40);
    let sk = new_key(2048, -40);
    sk.save_key(id);

    println!("Creating Bootstrapping key!");
    let bsk = new_bsk(&sk, None);

    println!("Saving Bootstrapping key!");
    bsk.save_key(id);

    println!("Creating Keyswitching key!");
    let ksk = new_ksk(&sk, &sk);

    println!("Saving Keyswitching key!");
    ksk.save_key(id);

}