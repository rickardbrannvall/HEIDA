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
    
    // $ add_vector sk_path prec padd lower upper n_vectors m_entries X11 X12 .... Xnm
    // target/release/add_vectors keys/def_80_512_1 6 4 0.0 1.0 2 2 0.5 0.3 0.7 0.4
    // output: key_load enc_time add_time dec_time Y1 ... Ym 
    
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
    let n_vectors: usize = args[6].parse().unwrap(); // 8;
    // if n is not power of 2 then fail
    let m_entries: usize = args[7].parse().unwrap(); // 4; 
    //if (args.len()-8) != m*n then fail

    //println!("# sk_path: {}", sk_path);
    //println!("# prec: {}", prec);
    //println!("# padd: {}", padd);
    //println!("# lower: {}", lower);
    //println!("# upper: {}", upper);
    //println!("# n_vectors: {}", n_vectors);
    //println!("# m_entries: {}", m_entries);
    
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    //println!("# Loading LWE key {} ...",sk0_LWE_path);
    let now = Instant::now();
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    let load_time = now.elapsed().as_micros(); //.as_millis();
    //println!("load_lwe_key {} {}", sk0_LWE_path, load_time);
        
    // create an encoder
    //println!("# create an encoder... \n");
    let enc = Encoder::new(lower, upper, prec, padd)?;

    let mut w = vec![];
    let mut w_enc = vec![];
    let mut enc_time: u128 = 0; 
    for i in 0..n_vectors {
        let mut v = vec![];
        for j in 0..m_entries {
            let x: f64 = args[8+i*m_entries+j].parse().unwrap();
            //println!("x: {}",x);
            v.push(x);
        }        
        //println!("v: {:?}",v);
        let now = Instant::now();
        let v_enc = VectorLWE::encode_encrypt(&sk0, &v, &enc)?;
        enc_time = now.elapsed().as_micros(); //.as_millis(); 
        w.push(v);
        w_enc.push(v_enc);
    }
    //println!("len(w): {}",w.len());
    
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
    
    let now = Instant::now();
    let a_enc = add(w_enc);
    let add_time = now.elapsed().as_micros(); //.as_millis();
    
    let now = Instant::now();
    let a = a_enc.decrypt_decode(&sk0).unwrap();
    let dec_time = now.elapsed().as_micros(); //as_millis();
    
    //println!("add: {:?}",a);

    println!("{} {} {} {} {:?}", load_time, enc_time, add_time, dec_time, a);

    /*
    def addition_tree(parts):
        n = len(parts)
        if n==1:
            return parts[0]
        elif n%2==0:
            return addition_tree(parts[:n//2]) + addition_tree(parts[n//2:]) 
        else:
            print("Error: length must power of 2")
            return None

    addition_tree([1,2,3,4])    
    */
    
    /*
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
    */
    
    Ok(())
}
