#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs;
use std::env;
use std::time::{Instant}; // Duration, 
use std::convert::TryInto;

use concrete::*;
use concrete_npe as npe;
use concrete_commons::numeric::Numeric;

fn main() -> Result<(), CryptoAPIError> {

    // def_80_256_1 => 7
    // def_80_512_1 => 17
    // def_80_1024_1 => 38
    // std_62_2048_1 => 60
    
    // Bootstrap single ciphertext
    //
    // input: sk_path base_log level in_prec in_padd out_prec times steps 
    // 
    // example:
    // target/release/boot_ranges keys/def_80_512_1 5 5 2 2 2 5 200
    // 
    // output: sk_path prec add_padd mul_padd out_padd value base_log level 
    // load_time, enc_time, dot_time, dec_time, answer, v1, n1, s1, label
        
    let args: Vec<String> = env::args().collect();
    
    let sk_path: String = args[1].parse().unwrap(); // String::from("keys/std_62_2048_1/"); 
    let base_log: usize = args[2].parse().unwrap(); // 4; 
    let level: usize = args[3].parse().unwrap(); // 4; 
    let in_prec: usize = args[4].parse().unwrap(); // 4; 
    let in_padd: usize = args[5].parse().unwrap(); // 4; 
    let out_prec: usize = args[6].parse().unwrap(); // 4; 
    let times: usize = args[7].parse().unwrap(); // 5; 
    let steps: usize = args[8].parse().unwrap(); // 200; 
        
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    

    let bsk00_path = format!("{}/bsk00_{}_{}.json",sk_path,base_log,level);
    let now = Instant::now();
    let bsk00 = LWEBSK::load(&bsk00_path);    
    let load_time = now.elapsed().as_micros(); //.as_millis();
    let mut load_time = format!("{}",load_time);
    
    let lower: f64 = -1.0;
    let upper: f64 = 1.0;
    
    //let in_prec: usize = 2; 
    //let out_prec: usize = 2; 
    //let in_padd = 1;
    //let out_padd = 1;
    
    //println!("# create input encoder ...");
    let tmp = Encoder::new(lower, upper, in_prec, in_padd);
    let enc_in = match tmp {
        Ok(data) => data,
        Err(_error) => {
            println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} enc_in", 
                sk0_LWE_path, base_log, level, in_prec, in_padd, "N/A", "N/A", 
                load_time, "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "N/A");
            return Ok(());
        }
    };   
    
    'out_enc:
    for out_padd in 1..=6 {
        //if out_padd <= 3 && in_padd <= 3 {  
        //    continue;
        //}
        
        //println!("# create output encoder ...");
        let tmp = Encoder::new(lower, upper, out_prec, out_padd);
        let enc_out = match tmp {
            Ok(data) => data,
            Err(_error) => {
                println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} enc_out", 
                    sk0_LWE_path, base_log, level, in_prec, in_padd, out_prec, out_padd, 
                    load_time, "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "N/A");
                load_time = format!("N/A");
                continue;
            }
        };  
                     
        for i in 0..times { 
          let d = 2f64/(steps as f64);
          for j in 0..=steps {   
            let value: f64 = -1.0 + (j as f64)*d;

            //println!("# encode x vector ...");
            let now = Instant::now();
            let x_enc = VectorLWE::encode_encrypt(&sk0, &[value], &enc_in)?;
            let enc_time = now.elapsed().as_micros(); //.as_millis();    

            //println!("# bootstrap identity ...");
            let a_enc = x_enc.bootstrap_nth_with_function(&bsk00, |x| x, &enc_out, 0).unwrap();
            let mul_time = now.elapsed().as_micros(); //.as_millis();

            //println!("# decrypt value ...");
            let now = Instant::now();
            let a = a_enc.decrypt_decode(&sk0); // .unwrap();
            let dec_time = now.elapsed().as_micros(); //as_millis();
            let a = match a {
                Ok(data) => data,
                Err(_error) => {
                    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} boot", 
                        sk0_LWE_path, base_log, level, in_prec, in_padd, out_prec, out_padd, 
                        load_time, value, enc_time, mul_time, dec_time, "N/A", "N/A", "N/A", "N/A");
                    load_time = format!("N/A");
                    continue 'out_enc;
                }
            };

            let v1 = &a_enc.variances[0];
            let n1 = npe::nb_bit_from_variance_99(*v1, <Torus as Numeric>::BITS as usize);
            let s1 = <Torus as Numeric>::BITS - n1;

            println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} ok", 
                    sk0_LWE_path, base_log, level, in_prec, in_padd, out_prec, out_padd, 
                    load_time, value, enc_time, mul_time, dec_time, a[0], v1, n1, s1);
            load_time = format!("N/A");
          }
        }      
    }
    
    Ok(())
}

