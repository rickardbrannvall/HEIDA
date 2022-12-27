/// file: main.rs
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // the encoder's parameters
    let min = -10.;
    let max = 10.;
    let precision = 8;
    let padding = 0;
    
    // create an Encoder instance
    let encoder = Encoder::new(min, max, precision, padding)?;
    
    // encode a single message
    let m1 = -6.276;
    let p1: Plaintext = encoder.encode_single(m1)?;
    
    // encode a vector of messages
    let m2 = vec![-6.276, 4.3, 0.12, -1.1, 7.78];
    let p2: Plaintext = encoder.encode(&m2)?;
    
    // decode the plaintext
    let o1: Vec<f64> = p1.decode()?;
    let o2: Vec<f64> = p2.decode()?;
    
    println!("{}", p1);
    println!("{}", p2);
    println!("m1 = {}, o1 = {:?}", m1, o1[0]);
    println!("m2 = {:?}, o2 = {:?}", m2, o2);
    Ok(())
}
