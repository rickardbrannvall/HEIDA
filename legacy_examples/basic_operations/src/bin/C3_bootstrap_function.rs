use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // encoders
    //let encoder_input = Encoder::new(-10., 10., 6, 1)?;
    let encoder_input = Encoder::new(-10., 10., 6, 2)?;
    let encoder_output = Encoder::new(0., 100., 6, 0)?;

    // secret keys
    //let sk_rlwe = RLWESecretKey::new(&RLWE128_1024_1);
    //let sk_in = LWESecretKey::new(&LWE128_630);
    //let sk_out = sk_rlwe.to_lwe_secret_key();
    //let sk_rlwe = RLWESecretKey::load("sk_rlwe.json").unwrap();    
    let sk_in = LWESecretKey::load("sk_in.json").unwrap();    
    let sk_out = LWESecretKey::load("sk_out.json").unwrap();    
    
    // bootstrapping key
    //let bsk = LWEBSK::new(&sk_in, &sk_rlwe, 5, 3);
    let bsk = LWEBSK::load("bsk.json");

    // messages
    let message: f64 = -5.;

    // encode and encrypt
    let c1 = LWE::encode_encrypt(&sk_in, message, &encoder_input)?;

    // bootstrap
    let c2 = c1.bootstrap_with_function(&bsk, |x| x * x, &encoder_output)?;

    // decrypt
    let output = c2.decrypt_decode(&sk_out)?;

    println!("before bootstrap: {}, after bootstrap: {}", message, output);

    Ok(())
}
