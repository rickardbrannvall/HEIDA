use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // generate a secret key
    let secret_key = LWESecretKey::new(&LWE128_630);

    // the two values to add
    let m1 = 8.2;
    let m2 = 5.6;
    
    // Encode in [0, 10[ with 8 bits of precision and 1 bit of padding
    let encoder = Encoder::new(0., 10., 8, 1)?;
    
    // encrypt plaintexts
    let mut c1 = LWE::encode_encrypt(&secret_key, m1, &encoder)?;
    let c2 = LWE::encode_encrypt(&secret_key, m2, &encoder)?;

    // add the two ciphertexts homomorphically, and store in c1
    c1.add_with_padding_inplace(&c2)?;

    // decrypt and decode the result
    let m3: f64 = c1.decrypt_decode(&secret_key)?;
    // print the result and compare to non-FHE addition
    println!("Real: {}, FHE: {}", m1 + m2, m3);

    // for vectors
    
    // message vectors to add
    let mv1: Vec<f64> = vec![1.2, 4.3, 0.11, 3.1, 6.7];
    let mv2: Vec<f64> = vec![7.0, 1.0, 8.2, 3.7, 9.4];

    // Encode in [0, 10[ with 8 bits of precision and 1 bit of padding
    let encoder = Encoder::new(0., 10., 8, 1)?;

    // encode encrypt
    let mut cv1 = VectorLWE::encode_encrypt(&secret_key, &mv1, &encoder)?;
    let cv2 = VectorLWE::encode_encrypt(&secret_key, &mv2, &encoder)?;

    // add ciphertext vectors element-wise
    cv1.add_with_padding_inplace(&cv2)?;

    let mv3: Vec<f64> = cv1.decrypt_decode(&secret_key)?;
    let tmp: Vec<f64> = (0..mv1.len()).map(|i| mv1[i] + mv2[i]).collect();

    
    println!("Real: {:?}, FHE: {:?}", tmp, mv3);
        
    Ok(())
}
