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
    
    // $ enc_vector sk_path encfile prec padd lower upper  m_entries X1 X2 .... Xm
    // target/release/enc_vector keys/def_80_512_1 temp/test.enc 6 4 0.0 1.0 4 0.5 0.3 0.7 0.4
    // output: sk_path, m_entries, enc_time, save_time 
    
    // println!("# Encrypt vector of length m");
    
    let args: Vec<String> = env::args().collect();
    //if args.len() < 1+8 {
    //    fail 
    //}
    
    let sk_path: String = args[1].parse().unwrap(); // String::from("keys/std_62_2048_1/"); 
    let encfile: String = args[2].parse().unwrap(); // String::from("temp/test.enc"); "sensor.enc";
    let prec: usize = args[3].parse().unwrap(); // 6;
    let padd: usize = args[4].parse().unwrap(); // 4; 
    let lower: f64 = args[5].parse().unwrap(); // 0.0;
    let upper: f64 = args[6].parse().unwrap(); // 2.0;
    let m_entries: usize = args[7].parse().unwrap(); // 4; 
    
    let offset = 8;

    println!("# sk_path: {}", sk_path);
    println!("# prec: {}", prec);
    println!("# padd: {}", padd);
    println!("# lower: {}", lower);
    println!("# upper: {}", upper);
    println!("# m_entries: {}", m_entries);
    
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    //println!("# Loading LWE key {} ...",sk0_LWE_path);
    let now = Instant::now();
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    let load_time = now.elapsed().as_micros(); //.as_millis();
    //println!("load_lwe_key {} {}", sk0_LWE_path, load_time);
        
    // create an encoder
    println!("# create an encoder... \n");
    let enc = Encoder::new(lower, upper, prec, padd)?;
    
    let mut v = vec![];
    for j in 0..m_entries {
        let x: f64 = args[offset+j].parse().unwrap();
        println!("x: {}",x);
        v.push(x);
    }        
    println!("v: {:?}",v);
    
    let now = Instant::now();
    let v_enc = VectorLWE::encode_encrypt(&sk0, &v, &enc)?;
    let enc_time: u128 = now.elapsed().as_micros(); //.as_millis(); 
    
    let now = Instant::now();
    v_enc.save(&encfile).unwrap();
    let save_time: u128 = now.elapsed().as_micros(); //.as_millis();        

    println!("{} {} {} {}", sk_path, m_entries, enc_time, save_time);
    
    Ok(())
}
