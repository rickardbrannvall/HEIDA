use lib::*;

fn main() {

    let id = "00001";

    println!("Creating Secret key!");
    // let sk = new_key(512, -40);
    let sk = new_key(1024, -40);
    // let sk = new_key(2048, -40);
    // let sk = new_key(4096, -40);
    sk.save_key(id);

    // let sk_out = new_key(1024, -40);
    let sk_out = new_key(2048, -40);
    // let sk_out = new_key(4096, -40);
    sk_out.save_key(&(id.to_owned()+"_out"));




    println!("Creating Bootstrapping key!");
    let bsk = new_bsk(&sk, Some(&sk_out));

    println!("Saving Bootstrapping key!");
    bsk.save_key(id);

    println!("Creating Keyswitching key!");
    let ksk = new_ksk(&sk_out, &sk);

    println!("Saving Keyswitching key!");
    ksk.save_key(id);

}