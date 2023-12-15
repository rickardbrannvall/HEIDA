/// file: main.rs
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // generate a secret key
    let secret_key = LWESecretKey::new(&LWE128_1024);
    // encoder
    let encoder = Encoder::new(100., 210., 8, 0)?;

    // encode and encrypt
    let message = 106.276;
    let mut ciphertext = LWE::encode_encrypt(&secret_key, message, &encoder)?;

    // addition between ciphertext and a constant
    let constant = 102.0;
    ciphertext.add_constant_static_encoder_inplace(constant)?;
    //ciphertext.add_constant_dynamic_encoder_inplace(constant)?;
    //ciphertext.add_inplace(constant)?;
    
    // decryption
    let output = ciphertext.decrypt_decode(&secret_key)?;
    println!("{} + {} = {}", message, constant, output);
    
    // vector operations
    // encode and encrypt
    let messages: Vec<f64> = vec![106.276, 104.3, 100.12, 101.1, 107.78];
    let mut ciphertext_vector = VectorLWE::encode_encrypt(
                                        &secret_key, &messages, &encoder)?;

    // addition between ciphertexts and constants
    let constants: Vec<f64> = vec![-4.9, 1.02, 4.6, 5.6, -3.2];
    ciphertext_vector.add_constant_dynamic_encoder_inplace(&constants)?;

    // decryption
    let outputs: Vec<f64> = ciphertext_vector.decrypt_decode(&secret_key)?;
    println!("{:?} + {:?} = {:?}", messages, constants, outputs);
    
    Ok(())
}
