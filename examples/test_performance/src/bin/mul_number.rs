#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs;
use std::env;
use std::time::{Instant}; // Duration, 
use std::convert::TryInto;

use concrete::*;

fn main() -> Result<(), CryptoAPIError> {

    // def_80_256_1 => 7
    // def_80_512_1 => 17
    // def_80_1024_1 => 38
    // std_62_2048_1 => 60
    
    // Multiplies a ciphertext by real constant
    // with padding for multiplication
    //
    // input: sk_path prec padd lower upper value mult max_mult 
    // 
    // example:
    // target/release/mul_integer keys/def_80_512_1 6 4 0.0 2.0 1.0 2.5 3.0 4
    // 
    // output: sk0_path, prec, padd, lower, upper, value, 
    // times, load_time, enc_time, add_time, dec_time, answer
    
    // println!("# Add n=2**k vectors of length m");
    
    let args: Vec<String> = env::args().collect();
    //if args.len() < 1+8 {
    //    fail 
    //}
    
    let sk_path: String = args[1].parse().unwrap(); // String::from("keys/std_62_2048_1/"); 
    let prec: usize = args[2].parse().unwrap(); // 6;
    let padd: usize = args[3].parse().unwrap(); // 4; 
    let lower: f64 = args[4].parse().unwrap(); // 0.0;
    let upper: f64 = args[5].parse().unwrap(); // 2.0;
    let value: f64 = args[6].parse().unwrap(); // 2.0;
    let mult: f64 = args[7].parse().unwrap(); // 5;
    let max_mult: f64 = args[8].parse().unwrap(); // 5;

    //println!("# sk_path: {}", sk_path);
    //println!("# prec: {}", prec);
    //println!("# padd: {}", padd);
    //println!("# lower: {}", lower);
    //println!("# upper: {}", upper);
    //println!("# value: {}", value);
    //println!("# times: {}", times);
    
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    //println!("# Loading LWE key {} ...",sk0_LWE_path);
    let now = Instant::now();
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    let load_time = now.elapsed().as_micros(); //.as_millis();
    //println!("load_lwe_key {} {}", sk0_LWE_path, load_time);
        
    // create an encoder
    //println!("# create an encoder... \n");
    let enc = Encoder::new(lower, upper, prec, padd)?;

    //let v = vec![&value];
    let now = Instant::now();
    let v_enc = VectorLWE::encode_encrypt(&sk0, &[value], &enc)?;
    //let z = vec![0f64];
    //let z_enc = VectorLWE::encode_encrypt(&sk0, &[0f64], &enc)?;
    let enc_time = now.elapsed().as_micros(); //.as_millis();     
    
    /*
    fn mul(enc_v: &VectorLWE, enc_z: &VectorLWE, base2: &i32, mult: &i32) -> VectorLWE { // f64 { //
        let mut w_enc = vec![];
        for i in 0..*base2 {
            if w_enc.len() < (*mult).abs().try_into().unwrap() {
                w_enc.push(enc_v);
            }
            else {
                w_enc.push(enc_z);
            }        
        }
        let mut res = add(w_enc);
        if *mult<0 {
            res =res.opposite_nth(0).unwrap();
        }
        return res;
    }
    
    fn add(enc_w: Vec<&VectorLWE>) -> VectorLWE { // f64 { //
        let n = enc_w.len();
        if n==1 {
            //return 1.0;
            return enc_w[0].clone();
        }
        else {
            //return add(enc_w[0..n/2].to_vec()) + add(enc_w[n/2..n].to_vec());
            let v1 = add(enc_w[0..n/2].to_vec());
            let v2 = add(enc_w[n/2..n].to_vec());
            let rs = v1.add_with_padding(&v2).unwrap();
            return rs;
        }
    }

    let constants: Vec<f64> = vec![-2.1, 1.4, 3.2];
    let max_constant: f64 = 4.;
    let nb_bit_padding = 4;
    ciphertext_vector.mul_constant_with_padding_inplace(&constants, max_constant, nb_bit_padding)?;
    
    let now = Instant::now();
    let a_enc = mul(&v_enc, &z_enc, &base2, &mult);
    let mul_time = now.elapsed().as_micros(); //.as_millis();

    */

    let mut mult_vec = vec![];
    mult_vec.push(mult);
    
    let now = Instant::now();
    let a_enc = v_enc.mul_constant_with_padding(&mult_vec, max_mult, padd).unwrap();
    let mul_time = now.elapsed().as_micros(); //.as_millis();
   
    let now = Instant::now();
    let a = a_enc.decrypt_decode(&sk0).unwrap();
    let dec_time = now.elapsed().as_micros(); //as_millis();
    
    //println!("add: {:?}",a);

    //println!("{} {} {} {} {}", load_time, enc_time, add_time, dec_time, a[0]);

    println!("{} {} {} {} {} {} {} {} {} {} {} {} {}", sk0_LWE_path, prec, padd, 
        lower, upper, value, mult, max_mult, load_time, enc_time, mul_time, dec_time, a[0]);
    
    Ok(())
}

