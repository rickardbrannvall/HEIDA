#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use concrete::*;
use std::fs;
use std::env;
use std::time::{Instant}; // Duration, 

fn main() -> Result<(), CryptoAPIError> {

    // note that key generation may take several hours 

    println!("# Parameters for bootstrap key creation");
    let mut sk_path = String::from("keys/std_62_2048_1/"); 
    let mut base_log: usize = 5;
    let mut level: usize = 3; 
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        sk_path = args[1].parse().unwrap();
        base_log = args[2].parse().unwrap();
        level = args[3].parse().unwrap();
    }
    println!("# sk_path: {}", sk_path);
    println!("# base_log: {}", base_log);
    println!("# level: {}", level);
    
    println!("# Loading LWE key...");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    let now = Instant::now();
    let sk0_LWE = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    let load_time = now.elapsed().as_millis();
    println!("load_lwe_key {} {}", sk0_LWE_path, load_time);
    
    println!("# Loading RLWE key...");
    let sk0_RLWE_path = format!("{}/sk0_RLWE.json",sk_path);
    let now = Instant::now();
    let sk0_RLWE = RLWESecretKey::load(&sk0_RLWE_path).unwrap();  
    let load_time = now.elapsed().as_millis();
    println!("load_rlwe_key {} {}", sk0_RLWE_path, load_time);
    
    println!("# Creating bootstrap key 00 ...");
    let bsk00_path = format!("{}/bsk00_{}_{}.json",&sk_path,base_log,level);
    let now = Instant::now();
    let bsk = LWEBSK::new(&sk0_LWE, &sk0_RLWE, base_log, level);
    let make_time = now.elapsed().as_millis();
    bsk.save(&bsk00_path);
    let save_time = now.elapsed().as_millis();
    println!("make_bs_key {} {}", bsk00_path, make_time);
    println!("save_bs_key {} {}", bsk00_path, save_time-make_time);
        
    Ok(())    
    
}
