use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    
    // secret keys
    let sk_rlwe = RLWESecretKey::new(&RLWE128_1024_1);
    sk_rlwe.save("sk_rlwe.json").unwrap();
    let sk_rlwe = RLWESecretKey::load("sk_rlwe.json").unwrap();    
    
    let sk_in = LWESecretKey::new(&LWE128_630);
    sk_in.save("sk_in.json").unwrap();
    let sk_in = LWESecretKey::load("sk_in.json").unwrap();    

    let sk_out = sk_rlwe.to_lwe_secret_key();
    sk_out.save("sk_out.json").unwrap();
    let sk_out = LWESecretKey::load("sk_out.json").unwrap();    
    
    // test
    let encoder = Encoder::new(-10., 10., 6, 1)?;
    let c1 = LWE::encode_encrypt(&sk_in, -5., &encoder)?;
    let m1 = c1.decrypt_decode(&sk_in)?;
    println!("{}",m1);
    let c2 = LWE::encode_encrypt(&sk_out, -5., &encoder)?;
    let m2 = c2.decrypt_decode(&sk_out)?;
    println!("{}",m2);
    
    // bootstrapping key
    let base_log: usize = 5;
    let level: usize = 3;
    let bsk = LWEBSK::new(&sk_in, &sk_rlwe, base_log, level);
    bsk.save("bsk.json");
    //let loaded_bsk = LWEBSK::load("bsk.json");
    
    Ok(())
}