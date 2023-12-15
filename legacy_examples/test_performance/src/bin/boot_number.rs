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
    
    // Bootstrap single ciphertext
    //
    // input: sk_path prec padd lower upper value base_log level  
    // 
    // example:
    // target/release/boot_number keys/def_80_512_1 6 4 -1.0 1.0 0.5 4 3
    // 
    // output: sk_path prec padd lower upper value base_log level 
    // load_time, enc_time, dot_time, dec_time, answer
        
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
    let base_log: usize = args[7].parse().unwrap(); // 4; 
    let level: usize = args[8].parse().unwrap(); // 4; 
    
    //println!("# sk_path: {}", sk_path);
    //println!("# prec: {}", prec);
    //println!("# padd: {}", padd);
    //println!("# lower: {}", lower);
    //println!("# upper: {}", upper);
    //println!("# value: {}", value);
    //println!("# times: {}", times);
    
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    //println!("# Loading LWE key {} ...",sk0_LWE_path);
    //let now = Instant::now();
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    //let load_time = now.elapsed().as_micros(); //.as_millis();
    //println!("load_lwe_key {} {}", sk0_LWE_path, load_time);

    let bsk00_path = format!("{}/bsk00_{}_{}.json",sk_path,base_log,level);
    //println!("# Loading bootstrap key {} ...",bsk00_path);
    let now = Instant::now();
    let bsk00 = LWEBSK::load(&bsk00_path);    
    let load_time = now.elapsed().as_micros(); //.as_millis();
    
    // create an encoder
    //println!("# create an encoder... \n");
    let enc = Encoder::new(lower, upper, prec, padd)?;
    
    let now = Instant::now();
    let v_enc = VectorLWE::encode_encrypt(&sk0, &[value], &enc)?;
    let enc_time = now.elapsed().as_micros(); //.as_millis();    
        
    let now = Instant::now();
    let a_enc = v_enc.bootstrap_nth_with_function(&bsk00, |x| x, &enc, 0).unwrap();
    let mul_time = now.elapsed().as_micros(); //.as_millis();
 
    let now = Instant::now();
    let a = a_enc.decrypt_decode(&sk0);
    let dec_time = now.elapsed().as_micros(); //as_millis();
    
    let a = match a {
        Ok(data) => data,
        Err(_error) => {
            println!("{} {} {} {} {} {} {} {} {} {} {} {} N/A", 
                sk0_LWE_path, prec, padd, lower, upper, value, base_log, level, 
                load_time, enc_time, mul_time, dec_time);
            return Ok(())
        }
    };    
    
    println!("{} {} {} {} {} {} {} {} {} {} {} {} {}", 
        sk0_LWE_path, prec, padd, lower, upper, value, base_log, level, 
        load_time, enc_time, mul_time, dec_time, a[0]);
    
    Ok(())
}

