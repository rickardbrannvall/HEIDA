use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // encoder
    let encoder = Encoder::new(-10., 10., 10, 4)?;

    // generate a secret key
    let secret_key = LWESecretKey::new(&LWE128_1024);

    // encrypt the message
    let message: f64 = 4.;
    let mut ciphertext = LWE::encode_encrypt(&secret_key, message, &encoder)?;
    
    // multiply in place by a 4-bit real constant
    let constant: f64 = 2.5;
    let max_constant: f64 = 3.;
    let nb_bit_padding = 4;
    ciphertext.mul_constant_with_padding_inplace(constant, 
                                        max_constant, nb_bit_padding)?;

    // decrypt
    let output: f64 = ciphertext.decrypt_decode(&secret_key)?;

    println!("{} * {} = {}", message, constant, output);

    // vector
    
    // encode and encrypt
    let messages: Vec<f64> = vec![6.1, 5.4, -2.7];
    let mut ciphertext_vector = VectorLWE::encode_encrypt(
                                    &secret_key, &messages, &encoder)?;

    // vector multiplication between ciphertext and constants
    let constants: Vec<f64> = vec![-2.1, 1.4, 3.2];
    let max_constant: f64 = 4.;
    let nb_bit_padding = 4;
    ciphertext_vector.mul_constant_with_padding_inplace(
                                    &constants, max_constant, nb_bit_padding)?;

    // decryption
    let outputs: Vec<f64> = ciphertext_vector.decrypt_decode(&secret_key)?;
    let tmp: Vec<f64> = (0..messages.len()
                ).map(|i| constants[i] * messages[i]).collect();
    
    println!("real: {:?}, FHE: {:?}", tmp, outputs);

    Ok(())
}