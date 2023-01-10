use concrete::*;

mod net;
use net::*;
use lib::*;

fn main(){

    let id = "00001";

    println!("Loading keys!\n");
    let sk0 = load_sk(id);
    let bsk = load_bsk(id);
    let ksk = load_ksk(id);

    let all_keys = (&sk0, &sk0, &bsk, &ksk);
    
    let n: usize = 12;
    let N = u32::pow(2, n as u32) as usize;
    
    println!("Loading data!\n");
    let data = load_data("data/two_weeks.csv", N);
    
    let net = Net::new(input_size, *h, output_size);

}