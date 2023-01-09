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
    
    let a = vec![-1.3 ,  0.05, -1.61, -0.53,  0.14, -1.89,  1.1 , -1.18,  0.61,
                1.18, -0.56,  0.19,  0.33,  0.46,  1.13, -1.6 , -1.07, -1.6 ,
                0.3 ,  1.88, -0.87, -0.58,  0.1 ,  0.2 ,  0.2 ,  1.36, -0.37,
               -0.64,  0.13, -1.68,  1.78,  1.67,  0.78, -1.46,  0.86, -0.15,
               -0.38,  1.79,  1.15, -0.85,  0.66, -0.84, -0.28,  0.1 , -0.28,
                0.77, -0.07, -0.4 , -1.35,  1.92,  0.74, -0.68,  1.03, -0.63,
               -1.26, -0.49,  1.1 ,  1.67,  0.26,  1.75,  1.57,  0.21,  0.39, 1.31];
    
    let b = vec![-0.09, -1.86, -0.08,  1.76,  0.9 ,  1.88,  1.12,  1.1];
    
    let r = vec![ 0.2 ,  0.43,  0.36,  0.12, -0.16,  0.42,  0.05, -0.58, -1.0,
               -0.21, -0.12,  0.37,  0.24, -0.31,  0.18, -0.48, -0.64, -0.76,
               -0.81, -0.52,  0.03,  0.58,  0.14,  0.16, -0.01, -0.25, -0.57,
               -0.07,  0.07,  0.57,  0.1 ,  0.22,  0.01, -0.39,  0.21, -0.3 ,
                0.31,  0.31,  0.47, -0.18,  0.71, -0.03, -0.38, -0.11, -0.51,
               -0.04,  0.23,  0.22,  0.13, -0.3 ,  0.78,  0.39, -0.06, -0.04,
               -0.12,  0.14, -0.06,  0.28,  0.36, -0.75,  0.24, -0.15, -0.21,
               -0.61, -0.12,  0.18,  0.08,  0.33, -0.46,  0.24, -0.29,  0.71,
               -0.41,  0.33, -0.41, -0.18,  0.12,  0.2 ,  0.06,  0.36, -0.17,
                0.57,  0.06, -0.11, -0.5 , -0.32, -0.64, -0.83, -0.6 ,  0.45,
                0.  , -0.24, -0.02,  0.45, -0.35, -0.15, -0.03, -0.05,  0.22,
               -0.13, -0.11,  0.22, -0.12, -0.52, -0.04,  0.29,  0.41, -0.67,
                0.61, -0.08,  0.3 ,  0.08, -0.4 ,  0.18, -0.47,  0.09,  0.12,
               -0.87, -0.23, -0.31, -0.29,  0.21, -0.18, -0.26, -0.46, -0.48,
               -0.23, -0.04,  0.33,  0.05, -0.14,  0.3 ,  0.38, -0.22,  0.58,
               -0.75,  0.09, -0.52, -0.27,  0.03, -0.31,  0.07,  0.67,  0.49,
               -0.  ,  0.67,  0.41, -0.83, -0.7 ,  0.55,  0.02,  0.32, -0.34,
               -0.59,  0.19,  0.36,  0.19,  0.11, -0.65, -0.16,  0.53,  0.42,
                0.12, -0.02,  0.56,  0.5 ,  0.4 , -0.32, -0.25,  0.01, -0.13,
               -0.51,  0.18,  0.62,  0.33, -0.33, -0.62,  0.43, -0.17,  0.08,
               -0.41, -0.38, -0.12,  0.04,  0.23,  0.24, -0.1 , -0.28,  0.08,
               -0.11, -0.22, -0.61, -0.82,  0.07, -0.05, -0.  , -0.  , -0.43,
               -0.01, -0.22,  0.79,  0.08, -0.27, -0.15,  0.58, -0.33,  0.21,
                0.19,  0.07, -0.43,  0.04,  0.08,  0.52,  0.09,  0.67,  0.25,
               -0.26, -0.16,  0.34, -0.1 ,  0.23, -0.93,  0.25,  0.61,  0.18,
                0.45,  0.18,  0.26, -0.2 , -0.36, -0.25, -0.3 , -0.48,  0.43,
                0.38, -0.31,  0.04, -0.39,  0.26,  0.33,  0.65,  0.26,  0.47,
                0.19, -0.12, -0.25,  0.09,  0.31, -0.22, -0.59, -0.57, -0.48,
                0.01,  0.26, -0.22,  0.24,  0.26, -0.57,  0.96,  0.51, -1.  ,
                1.  ,  0.49,  0.11, -0.37, -0.18, -0.11, -0.32, -0.85,  0.23,
               -0.3 , -0.06,  0.02, -0.06, -0.41, -0.28, -0.33, -0.01, -0.61,
                0.51, -0.07, -0.86,  0.21,  0.07,  0.17, -0.09, -0.41, -0.29,
               -0.1 , -0.43, -0.35,  0.24, -0.2 ,  0.64, -0.88, -0.26,  0.34,
                0.56, -0.63,  0.28, -0.1 ,  0.13,  0.43,  0.2 ,  0.4 , -0.02,
                0.11,  0.09,  0.26, -0.45,  0.61,  0.06, -0.36,  0.18, -0.13,
               -0.25, -0.2 ,  0.22,  0.04, -0.41, -1.  , -0.56, -0.14,  0.18,
                0.21, -0.01, -0.34,  0.37, -0.15, -0.03,  0.06, -0.28,  0.07,
                0.44,  0.57,  0.14, -0.02, -0.2 , -0.09,  0.08, -0.15,  0.79,
                0.22, -0.52, -0.06,  0.34, -0.5 , -0.01,  0.38, -0.09,  0.76,
               -0.06,  0.03,  0.2 , -0.08,  0.49,  0.23, -0.38, -0.42, -0.4 ,
                0.15,  0.48,  0.47,  0.18, -0.34, -0.01, -0.35,  0.44,  0.2 ,
                0.08,  0.17, -0.28,  0.67, -0.01,  0.48,  0.47,  0.05,  0.06,
               -0.44, -0.2 , -0.43, -0.42,  0.49,  0.63,  0.49, -0.41, -0.83,
                0.49, -0.01,  0.53, -0.22,  0.3 ,  0.75, -0.6 , -0.42, -0.01,
                0.07, -0.53, -0.03,  0.14,  0.18,  0.56,  0.6 , -0.25,  0.73,
                0.33,  0.03,  0.29, -0.73,  0.23,  0.54, -0.03, -0.44,  0.2 ,
               -0.04, -0.46, -0.16, -0.73, -0.42,  0.35,  0.19,  0.36,  0.08,
               -0.29,  0.3 , -0.1 , -0.  ,  0.46, -0.12,  0.38, -0.55, -0.21,
               -0.01,  0.52,  0.04, -0.48,  0.33,  0.2 , -0.2 , -0.46,  0.45,
                0.02,  0.32,  0.42, -0.44,  0.43,  0.3 , -0.16, -0.19,  0.53,
                0.01, -0.17, -0.12, -0.54, -0.5 , -0.2 , -0.24, -0.11,  0.89,
               -0.23, -0.21,  0.25, -0.01, -0.49, -0.02, -0.11,  0.27, -0.45,
               -0.29, -0.72,  0.22, -0.54, -0.15, -0.23,  0.41, -0.38, -0.46,
               -0.57,  0.29,  0.47, -0.18, -0.41, -0.36, -0.61,  0.26, -0.19,
                0.63,  0.47, -0.03,  0.34,  0.18,  0.65,  0.03, -0.33, -0.17,
                0.01, -0.66, -0.83,  0.07, -0.68,  0.  , -0.03,  0.04, -0.03,
               -0.25, -1.  ,  0.07,  0.41, -0.02, -0.8 , -0.05, -0.13];

    pub const PI: f64 = 3.1415926535898f64;
    
    fn normatan(x: f64) -> f64 {
        return x.atan()/(PI/2.0);
    }
    
    fn add(enc_w: Vec<VectorLWE>) -> VectorLWE { // f64 { //
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
    
    // Bootstrap single neuron
    //
    // input: sk_path prec base_log level  
    // 
    // example:
    // target/release/atan_neuron keys/def_80_512_1 2 5 5
    // 
    // output: sk_path prec add_padd mul_padd out_padd value base_log level 
    // load_time, enc_time, dot_time, dec_time, answer, v1, n1, s1, label
        
    let args: Vec<String> = env::args().collect();
    
    let sk_path: String = args[1].parse().unwrap(); // String::from("keys/std_62_2048_1/"); 
    let prec: usize = args[2].parse().unwrap(); // 6;
    let base_log: usize = args[3].parse().unwrap(); // 4; 
    let level: usize = args[4].parse().unwrap(); // 4; 
        
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    

    let bsk00_path = format!("{}/bsk00_{}_{}.json",sk_path,base_log,level);
    let now = Instant::now();
    let bsk00 = LWEBSK::load(&bsk00_path);    
    let load_time = now.elapsed().as_micros(); //.as_millis();
    let mut load_time_text = format!("{}",load_time);
    
    let max_mult = 2.0; 
    let lower: f64 = -1.0;
    let upper: f64 = 1.0;
    
    //let prec: usize = 2; 
    //let mul_padd: usize = 3; // mul_padd is also required
    //let add_padd: usize = 3; // mul_padd is also required
    
    //let base2: usize = 2usize.pow(add_padd as u32); 
    //let tot_padd = mul_padd + add_padd + 1;
    //let out_padd = 1;
    
    for mul_padd in 3..=6 {
     for add_padd in 1..=6 {
      let base2: usize = 2usize.pow(add_padd as u32); 
      let tot_padd = mul_padd + add_padd + 1;   
      for out_padd in 1..=tot_padd { 
       for label in 0..8 {
        let mut l0 = label*base2;
        let mut l1 = (label+1)*base2-1;
        //println!("l: {},{}",l0,l1);
        let x_vec = &r[l0..=l1];
        l0 = l0%64;
        l1 = l1%64;
        //println!("l: {},{}",l0,l1);
        let coeff = &a[l0..=l1];
        let value: f64 = x_vec.iter().zip(coeff.iter()).map(|(a, b)| a * b).sum();
        let value: f64 = normatan(value);
        //println!("l: {},{},{}",x_vec.len(),coeff.len(),value);

        //println!("# create input encoder ...");
        let tmp = Encoder::new(lower, upper, prec, tot_padd);
        let enc = match tmp {
            Ok(data) => data,
            Err(_error) => {
                println!("{} {} {} {} {} {} {} {} {} {} {} {} N/A N/A N/A N/A {} enc", 
                    sk0_LWE_path, prec, add_padd, mul_padd, out_padd, value, base_log, level, 
                    load_time_text, "N/A", "N/A", "N/A", label);
                continue;
            }
        };
        
        //println!("# create output encoder ...");
        let tmp = Encoder::new(lower, upper, prec, out_padd);
        let enc_out = match tmp {
            Ok(data) => data,
            Err(_error) => {
                println!("{} {} {} {} {} {} {} {} {} {} {} {} N/A N/A N/A N/A {} enc_out", 
                    sk0_LWE_path, prec, add_padd, mul_padd, out_padd, value, base_log, level, 
                    load_time_text, "N/A", "N/A", "N/A", label);
                continue;
            }
        };
        
        //println!("# encode x vector ...");
        let now = Instant::now();
        let x_enc = VectorLWE::encode_encrypt(&sk0, &x_vec, &enc)?;
        let enc_time = now.elapsed().as_micros(); //.as_millis();    

        //println!("# mul x_vec and coeff ...");
        let now = Instant::now();
        let tmp = x_enc.mul_constant_with_padding(&coeff, max_mult, mul_padd);
        let d_enc = match tmp {
            Ok(data) => data,
            Err(_error) => {
                println!("{} {} {} {} {} {} {} {} {} {} {} {} N/A N/A N/A N/A {} mul", 
                    sk0_LWE_path, prec, add_padd, mul_padd, out_padd, value, base_log, level, 
                    load_time_text, enc_time, "N/A", "N/A", label);
                continue;
            }
        };
        
        //println!("# split and sum product ...");
        let mut d_vec = vec![];
        for j in 0..base2 {
            let tmp = d_enc.extract_nth(j).unwrap();
            d_vec.push(tmp);
        }   
        let d_sum = add(d_vec);
        
        //println!("# bootstrap activation ...");
        let a_enc = d_sum.bootstrap_nth_with_function(&bsk00, normatan, &enc_out, 0).unwrap();
        let mul_time = now.elapsed().as_micros(); //.as_millis();

        //println!("# decrypt value ...");
        let now = Instant::now();
        let a = a_enc.decrypt_decode(&sk0); // .unwrap();
        let dec_time = now.elapsed().as_micros(); //as_millis();
        let a = match a {
            Ok(data) => data,
            Err(_error) => {
                println!("{} {} {} {} {} {} {} {} {} {} {} {} N/A N/A N/A N/A {} boot", 
                    sk0_LWE_path, prec, add_padd, mul_padd, out_padd, value, base_log, level, 
                    load_time_text, enc_time, mul_time, dec_time, label);
                continue;
            }
        };

        let v1 = &a_enc.variances[0];
        //println!("var {:?}", v1);

        let n1 = npe::nb_bit_from_variance_99(*v1, <Torus as Numeric>::BITS as usize);
        //println!("bits {:?}", n1);  

        let s1 = <Torus as Numeric>::BITS - n1;
        //println!("free {:?}", s1);             

        println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} ok", 
            sk0_LWE_path, prec, add_padd, mul_padd, out_padd, value, base_log, level, 
            load_time_text, enc_time, mul_time, dec_time, a[0], v1, n1, s1, label);
        //return Ok(()) 
       }     
      }      
     }      
    }
    
    /*
    for j in 0..times {
        for i in 0..11 {
            let value = lower + (upper-lower)/10f64*(i as f64);
            let now = Instant::now();
            let v_enc = VectorLWE::encode_encrypt(&sk0, &[value], &enc_in)?;
         
         let enc_time = now.elapsed().as_micros(); //.as_millis();    

            let now = Instant::now();
            let a_enc = v_enc.bootstrap_nth_with_function(&bsk00, |x| x, &enc_out, 0).unwrap();
            let mul_time = now.elapsed().as_micros(); //.as_millis();

            let now = Instant::now();
            let a = a_enc.decrypt_decode(&sk0);
            let dec_time = now.elapsed().as_micros(); //as_millis();
            
            let a = match a {
                Ok(data) => {                    
                    let v1 = &a_enc.variances[0];
                    //println!("var {:?}", v1);

                    let n1 = npe::nb_bit_from_variance_99(*v1, <Torus as Numeric>::BITS as usize);
                    //println!("bits {:?}", n1);  

                    let s1 = <Torus as Numeric>::BITS - n1;
                    //println!("free {:?}", s1);             
                    
                    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", 
                        sk0_LWE_path, prec, padd, lower, upper, value, base_log, level, 
                        load_time_text, enc_time, mul_time, dec_time, data[0], v1, n1, s1);
                    //return Ok(())        
                },
                Err(_error) => {
                    println!("{} {} {} {} {} {} {} {} {} {} {} {} N/A N/A N/A N/A", 
                        sk0_LWE_path, prec, padd, lower, upper, value, base_log, level, 
                        load_time_text, enc_time, mul_time, dec_time);
                    return Ok(())
                }
            };
            load_time_text = format!("N/A");
        }
    }
    */

    Ok(())
}

