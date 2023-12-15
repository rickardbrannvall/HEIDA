#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs;
use std::env;
use std::time::{Instant}; // Duration, 
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {

    // def_80_256_1 => 7
    // def_80_512_1 => 17
    // def_80_1024_1 => 38
    // std_62_2048_1 => 60
        
    println!("# Parameters for basic tests");
    let mut sk_path = String::from("keys/std_62_2048_1/"); 
    let mut prec: usize = 6;
    let mut padd: usize = 4; 
    let mut value: f64 = 1.0;
    let mut lower: f64 = 0.0;
    let mut upper: f64 = 2.0;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        sk_path = args[1].parse().unwrap();
    }
    if args.len() > 3 {
        prec = args[2].parse().unwrap();
        padd = args[3].parse().unwrap();
    }
    if args.len() > 4 {
        value = args[4].parse().unwrap();
    }
    if args.len() > 6 {
        lower = args[5].parse().unwrap();
        upper = args[6].parse().unwrap();
    }
    println!("# sk_path: {}", sk_path);
    println!("# prec: {}", lower);
    println!("# padd: {}", upper);
    println!("# value: {}", value);
    println!("# lower: {}", lower);
    println!("# upper: {}", upper);
    
    println!("# Loading LWE key...");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    let now = Instant::now();
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    let load_time = now.elapsed().as_millis();
    println!("load_lwe_key {} {}", sk0_LWE_path, load_time);
        
    // create an encoder
    println!("# create an encoder... \n");
    let enc = Encoder::new(lower, upper, prec, padd)?;

    let m0: Vec<f64> = vec![value];
    println!("# plaintext value {:?}\n", m0);
    
    let c0 = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    println!("# encrypted value {:?}", c0.decrypt_decode(&sk0).unwrap());
    c0.pp();
    
    let constants: Vec<f64> = vec![1.0];
    
    let mut ct = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    ct.add_constant_static_encoder_inplace(&constants)?; 
    println!("add constant one {:?}", ct.decrypt_decode(&sk0).unwrap());
    ct.pp();   

    let mut ct = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    //ct.add_with_padding_inplace(&ct)?;
    let mut mt = value;
    for i in 1..=padd {
        ct = ct.add_with_padding(&ct)?;
        mt = mt + mt;
        println!("add with padding {} {} {:?}", i, mt, ct.decrypt_decode(&sk0).unwrap());
        ct.pp();
    }

    ct = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    ct.add_with_new_min_inplace(&c0, &vec![0.0])?;
    println!("add with new min {:?}", ct.decrypt_decode(&sk0).unwrap());
    ct.pp();     

    let max_constant: f64 = 2.0;
    let nb_bit_padding = 4;

    ct = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    ct.mul_constant_with_padding_inplace(&constants, max_constant, nb_bit_padding)?;
    println!("mul constant one {:?}", ct.decrypt_decode(&sk0).unwrap());
    ct.pp();      

    ct = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    ct.opposite_nth_inplace(0).unwrap();
    println!("negation of val {:?}", ct.decrypt_decode(&sk0).unwrap());
    ct.pp();     
    
    Ok(())
}
