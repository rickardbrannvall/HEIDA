#![allow(non_snake_case)]
use concrete::*;
use std::fs;
use std::env;
use std::time::{Instant}; // Duration, 

fn main() -> Result<(), CryptoAPIError> {

    // note that key generation may take several hours 

    println!("# Parameters for secret key creation");
    let mut poly_size: usize = 2048; 
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        poly_size =  args[1].parse().unwrap();
    }
    println!("# poly_size {}", poly_size);
    
    let mut rlwe_params0: RLWEParams = RLWEParams{polynomial_size: 2048, dimension: 1, log2_std_dev: -62};
    let mut name = "std_62_2048_1";
    if poly_size == 256 {
        rlwe_params0 = RLWE80_256_1;
        name = "def_80_256_1";    
    }
    else
    if poly_size == 512 {
        rlwe_params0 = RLWE80_512_1;
        name = "def_80_512_1";    
    }
    else
    if poly_size == 1024 {
        rlwe_params0 = RLWE80_1024_1;
        name = "def_80_1024_1";
    }
        
    let path = format!("keys/{}",name);
    fs::create_dir_all(&path).unwrap();
    
    println!("# Creating RLWE secret key ...");
    let sk0_RLWE_path = format!("{}/sk0_RLWE.json",&path);
    let now = Instant::now();
    let sk0_RLWE = RLWESecretKey::new(&rlwe_params0);     
    let make_time = now.elapsed().as_millis();
    sk0_RLWE.save(&sk0_RLWE_path).unwrap();
    let save_time = now.elapsed().as_millis();
    println!("make_rlwe_key {} {}", sk0_RLWE_path, make_time);
    println!("save_rlwe_key {} {}", sk0_RLWE_path, save_time-make_time);
    
    println!("# Creating basis LWE secret key ...");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",&path);
    let now = Instant::now();
    let sk0_LWE = sk0_RLWE.to_lwe_secret_key();
    let make_time = now.elapsed().as_millis();
    sk0_LWE.save(&sk0_LWE_path).unwrap();
    let save_time = now.elapsed().as_millis();
    println!("make_lwe_key {} {}", sk0_LWE_path, make_time);
    println!("save_lwe_key {} {}", sk0_LWE_path, save_time-make_time);
       
    Ok(())    
    
}

