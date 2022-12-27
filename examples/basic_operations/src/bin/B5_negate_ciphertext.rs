/// file: main.rs
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // encoder
    let encoder = Encoder::new(-10., 10., 8, 2)?;

    // generate a secret key
    let secret_key = LWESecretKey::new(&LWE128_1024);

    // the message to negate
    let message: f64 = 5.46;

    // encode and encrypt
    let mut ciphertext = LWE::encode_encrypt(&secret_key, message, &encoder)?;

    // compute the opposite of the ciphertext
    ciphertext.opposite_inplace()?;

    // decryption
    let output: f64 = ciphertext.decrypt_decode(&secret_key)?;

    // check the value computed
    println!("opposite({}) = {}", message, output);
    
    // vector
    let mv1: Vec<f64> = vec![6.1, 5.4, -2.7];
    let mut cv1 = VectorLWE::encode_encrypt(&secret_key, &mv1, &encoder)?;
    println!("{:?}", mv1);

    cv1.opposite_nth_inplace(0)?;
    let mv2: Vec<f64> = cv1.decrypt_decode(&secret_key)?;
    println!("{:?}", mv2);
    
    Ok(())
}