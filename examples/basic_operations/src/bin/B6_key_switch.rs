/// file: main.rs
use concrete::*;
//use concrete::LWEKSK;
//use crate::lwe_ksk::LWEKSK;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> Result<(), CryptoAPIError> {
    // encoder
    let encoder = Encoder::new(100., 110., 5, 0)?;

    // generate two secret keys
    let secret_key_before = LWESecretKey::new(&LWE128_1024);
    let secret_key_after = LWESecretKey::new(&LWE128_630);

    // generate the key switching key
    let ksk = LWEKSK::new(&secret_key_before, &secret_key_after, 2, 6);

    // a list of messages that we encrypt
    let messages: Vec<f64> = vec![106.276, 104.3, 100.12, 101.1, 107.78];
    println!("{:?}",messages);
        
    let ciphertext_before = VectorLWE::encode_encrypt(
                                        &secret_key_before, &messages, &encoder)?;
    print_type_of(&ciphertext_before);

    // key switch
    let ciphertext_after = ciphertext_before.keyswitch(&ksk)?;
    print_type_of(&ciphertext_after);

    // decryption
    //let decryptions: Vec<f64> = ciphertext_before.decrypt_decode(&secret_key_before)?;
    let decryptions: Vec<f64> = ciphertext_after.decrypt_decode(&secret_key_after)?;

    println!("{:?}",decryptions);
    
    
    Ok(())
}