use concrete::*;

mod functions;
use functions::*;
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
    
    let enc_gvp = Encoder::new(0., 400., 11, 2).unwrap();
    let enc_mean = Encoder::new(0., 400., 11, n+1).unwrap();
    let enc_ptir = Encoder::new(0., 400., 11, 1).unwrap();
    
    // Vad om man får dGlucose som data också?
    let enc_data_gvp = VectorLWE::encode_encrypt(&sk0, &data, &enc_gvp).unwrap();
    let enc_data_mean = VectorLWE::encode_encrypt(&sk0, &data, &enc_mean).unwrap();
    let enc_data_ptir = VectorLWE::encode_encrypt(&sk0, &data, &enc_ptir).unwrap();
        
    
    println!("Data loaded and encrypted!\n");
    
    let (gvp, mg, ptir, hypo, pgs) = pgs((&enc_data_gvp, &enc_data_mean, &enc_data_ptir), all_keys);
    let (gvp, mg) = (gvp.decrypt_decode(&sk0).unwrap(), mg.decrypt_decode(&sk0).unwrap());
    let (ptir, hypo) = (ptir.decrypt_decode(&sk0).unwrap(), hypo.decrypt_decode(&sk0).unwrap());
    println!(" \n \n \n//---- Results ----//");
    
    println!("GVP = {:?}, MG = {:?}, PTIR = {:?}, H = {:?}, PGS = {:?}", gvp, mg, ptir, hypo, pgs.decrypt_decode(&sk0).unwrap());

}