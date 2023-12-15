use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // encoder
    let encoder = Encoder::new(-30., 30., 8, 0)?;

    // generate a secret key
    let secret_key = LWESecretKey::new(&LWE128_1024);

    // encrypt the message
    let message: f64 = 6.2;
    let mut ciphertext = LWE::encode_encrypt(&secret_key, message, &encoder)?;
    // multiply in place by an integer constant
    let constant: i32 = -4;
    ciphertext.mul_constant_static_encoder_inplace(constant)?;

    // decrypt
    let output: f64 = ciphertext.decrypt_decode(&secret_key)?;

    println!("{} * {} = {}", message, constant, output);

    // vector

    // encode and encrypt
    let messages: Vec<f64> = vec![6.1, 5.4, -2.7];
    let mut ciphertext_vector = VectorLWE::encode_encrypt(
                                    &secret_key, &messages, &encoder)?;

    // vector multiplication between ciphertext and constants
    let constants: Vec<i32> = vec![-4, 5, 3];
    ciphertext_vector.mul_constant_static_encoder_inplace(&constants)?;

    // decryption
    let outputs: Vec<f64> = ciphertext_vector.decrypt_decode(&secret_key)?;
    let tmp: Vec<f64> = (0..messages.len()
                ).map(|i| constants[i] as f64 * messages[i]).collect();
  
    println!("real: {:?}, FHE: {:?}", tmp, outputs);
        
    Ok(())
}
