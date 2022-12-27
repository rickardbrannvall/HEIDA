use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // params
    /*
    let (min_1, max_1): (f64, f64) = (-150., 204.);
    let min_2: f64 = 30.;
    let max_2: f64 = min_2 + max_1 - min_1;

    let precision = 4;
    let padding = 2;
    let level: usize = 3;
    let base_log: usize = 3;

    // encoder
    let encoder_1 = Encoder::new(min_1, max_1, precision, padding).unwrap();
    let encoder_2 = Encoder::new(min_2, max_2, precision, padding).unwrap();

    // generate a secret key
    let secret_key = LWESecretKey::new(&LWE128_650);

    // two lists of messages
    let message_1: f64 = -127.;
    let message_2: f64 = 72.7;

    // generate secret keys
    let rlwe_secret_key = RLWESecretKey::new(&RLWE128_1024_1);
    let secret_key_before = LWESecretKey::new(&LWE128_630);
    let secret_key_after = rlwe_secret_key.to_lwe_secret_key();

    // bootstrapping key
    let bootstrapping_key =
        LWEBSK::new(&secret_key_before, &rlwe_secret_key, base_log, level);

    // a list of messages that we encrypt
    let ciphertext_1 =
        LWE::encode_encrypt(&secret_key_before, message_1, &encoder_1).unwrap();

    let ciphertext_2 =
        LWE::encode_encrypt(&secret_key_before, message_2, &encoder_2).unwrap();

    let ciphertext_out = ciphertext_1
        .mul_from_bootstrap(&ciphertext_2, &bootstrapping_key)
        .unwrap();
    */

    let encoder = Encoder::new(0., 10., 6, 2)?;

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
    
    let (m1, m2): (f64, f64) = (1.5, 2.);
    
    let ciphertext_1 =
        LWE::encode_encrypt(&sk_in, m1, &encoder).unwrap();

    let ciphertext_2 =
        LWE::encode_encrypt(&sk_in, m2, &encoder).unwrap();

    let ciphertext_out = ciphertext_1
        .mul_from_bootstrap(&ciphertext_2, &bsk)
        .unwrap();

    let output = ciphertext_out.decrypt_decode(&sk_out)?;
    println!("before bootstrap: {}, after bootstrap: {}", m1*m2, output);

    
    Ok(())
}