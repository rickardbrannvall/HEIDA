#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::{Array, Array2};
use ndarray_csv::{Array2Reader, Array2Writer};
use std::error::Error;
use std::fs::File;

use ndarray::prelude::*;
use std::iter::FromIterator; 

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
    
    // Dot product of ciphertext vector by real vector
    // each of length 8
    // with padding for multiplication
    //
    // input: sk_path prec padd lower upper max_mult  
    // 
    // example:
    // target/release/logres keys/def_80_512_1 6 4 -1.0 1.0 5.0
    // 
    // output: sk_path prec padd lower upper max_mult value answer
        
    let args: Vec<String> = env::args().collect();
    
    let sk_path: String = args[1].parse().unwrap(); // String::from("keys/std_62_2048_1/"); 
    let prec: usize = args[2].parse().unwrap(); // 6;
    let padd: usize = args[3].parse().unwrap(); // 4; 
    let lower: f64 = args[4].parse().unwrap(); // 0.0;
    let upper: f64 = args[5].parse().unwrap(); // 2.0;
    //let max_mult: f64 = args[6].parse().unwrap(); // 2.0;
    
    let exp2: usize = 3; 
    let base2: usize = 8; 

    // Read an param array from file
    //let file = File::open("params.csv").expect("What the gammastay");
    //let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    //let params: Array2<f64> = reader.deserialize_array2((1, 9)).unwrap();    
    //println!("shape: {}",params.len());
    //println!("params: {:?}",params);
    
    let bias:f64 = 1.2169812;
    //println!("bias: {}",bias);

    let weights = array![ 0.35881346,  1.0923927 ,  0.31851858, -0.02591134,  0.46353412,
          1.0900406 ,  0.12631564,  0.82225555];
    //println!("weights: {:?}",weights);

    let w = vec![ 0.35881346,  1.0923927 ,  0.31851858, -0.02591134,  0.46353412,
          1.0900406 ,  0.12631564,  0.82225555];
    //println!("w: {:?}",w);
    
    let max_mult: f64 = 1.2;
    
    // Read an data array from file
    let file = File::open("data/X_test.csv").expect("What the gammastay");
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let data: Array2<f64> = reader.deserialize_array2((200, 8)).unwrap();    
    //println!("shape: {},{}",data.nrows(),data.ncols());
     
    let sk0_LWE_path = format!("{}/sk0_LWE.json",sk_path);
    //println!("# Loading LWE key {} ...",sk0_LWE_path);
    let now = Instant::now();
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    let load_time = now.elapsed().as_micros(); //.as_millis();
    //println!("load_lwe_key {} {}", sk0_LWE_path, load_time);
        
    // create an encoder
    //println!("# create an encoder... \n");
    let enc = Encoder::new(lower, upper, prec, padd+exp2)?;

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
    
    for x in data.outer_iter() {
        let x_vec = x.as_slice().unwrap();
        //println!("x_vec: {:?}",x_vec);
        let x_enc = VectorLWE::encode_encrypt(&sk0, &x_vec, &enc)?;
        let value = x.dot(&weights) + bias;
        //println!("value: {}",value);
        let m_enc = x_enc.mul_constant_with_padding(&w, max_mult, padd).unwrap();
        let mut m_vec = vec![];
        for j in 0..base2 {
            m_vec.push(m_enc.extract_nth(j).unwrap());
        }   
        let d_enc = add(m_vec);
        let a_enc = d_enc.add_constant_static_encoder(&[bias]).unwrap();
        let answer = a_enc.decrypt_decode(&sk0).unwrap()[0];
        //println!("answer: {}",answer);
        println!("{} {} {} {} {} {} {}", 
                    sk0_LWE_path, prec, padd, lower, upper, value, answer);
        //break;
    }
            
    
    Ok(())
}

