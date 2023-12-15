/// file: main.rs
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // generate a secret key and save it
    let secret_key = LWESecretKey::new(&LWE128_630);
    //secret_key.save("my_very_secret_key.json");

    // create an encoder
    let encoder = Encoder::new(-10., 10., 8, 0)?;

    // a list of messages
    let messages: Vec<f64> = vec![-6.276, 4.3, 0.12, -1.1, 7.78];

    // encode and encrypt message vector
    let ciphertext = VectorLWE::encode_encrypt(&secret_key, &messages, &encoder)?;

    // decrypt
    let outputs: Vec<f64> = ciphertext.decrypt_decode(&secret_key)?;

    println!("{:?}", outputs);
    Ok(())
}
