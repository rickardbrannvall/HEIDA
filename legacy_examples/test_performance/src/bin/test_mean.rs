#![allow(non_snake_case)]
use concrete::*;
use std::env;

pub struct ConcreteContext {
    //enc_full: Encoder,
    bsk: LWEBSK,
    sk: LWESecretKey
}

impl ConcreteContext {
     
    pub fn weighted_mean_of_pair(&mut self, x1: &VectorLWE, x2: &VectorLWE, phi: f64, enc: &Encoder) -> VectorLWE {
        // assert phi is in range [0.0,1.0] 
        let scale: f64 = if phi > 0.5 {phi} else {1.-phi}; 
        let min = scale*enc.o;
        let max = scale*(enc.o + enc.delta);
        let enc_part = Encoder::new(min, max, enc.nb_bit_precision, enc.nb_bit_padding).unwrap();
        let term1 = (*x1).bootstrap_nth_with_function(&self.bsk, |x| phi * x, &enc_part, 0).unwrap();
        let term2 = (*x2).bootstrap_nth_with_function(&self.bsk, |x| (1.-phi) * x, &enc_part, 0).unwrap();
        let res = term1.add_with_padding(&term2).unwrap();
        return res;
    }       
    
    pub fn weighted_mean_recursion(&mut self, x: &[VectorLWE], enc: &Encoder) -> VectorLWE{
        // assert n>0
        let n = x.len();
        let m = n/2;
        if n == 1 {
            println!("n, m: {}, {}", n, m);
            return x[0].clone();
        }
        let x_i = self.weighted_mean_recursion(&x[..m], &enc);
        let x_j = self.weighted_mean_recursion(&x[m..], &enc);
        let phi: f64 = (m as f64)/(n as f64);
        println!("n, m, phi: {}, {}, {}", n, m, phi);
        return self.weighted_mean_of_pair(&x_i, &x_j, phi, &enc);
    }
    
    pub fn weighted_mean_of_many(&mut self, x: &[VectorLWE], enc: &Encoder) -> VectorLWE{
        let res = self.weighted_mean_recursion(&x, &enc);
        return res.bootstrap_nth_with_function(&self.bsk, |x| x, &enc, 0).unwrap();           
    }
}


fn main() -> Result<(), CryptoAPIError> {

    let path = "keys/80_1024_1";
    let mut base_log: usize = 6;
    let mut level: usize = 4; 
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        base_log =  args[1].parse().unwrap();
        level = args[2].parse().unwrap();
    }
    println!("base_log {}", base_log);
    println!("level {}", level);    
    let path = format!("{}_{}_{}",path,base_log,level);
    println!("key path {}", path);    
    
    //println!("loading LWE sk 0... \n");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    //let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    let bsk00_path = format!("{}/bsk00_LWE.json",path);
    //let bsk00 = LWEBSK::load(&bsk00_path);

    let enc = Encoder::new(0., 200., 3, 2).unwrap();
    
    let mut context = ConcreteContext{
        //enc_full: Encoder::new(0., 200., 3, 2).unwrap(),
        bsk: LWEBSK::load(&bsk00_path),
        sk: LWESecretKey::load(&sk0_LWE_path).unwrap()    
    };    
        
    // test mean_of_pair
    
    let x1: Vec<f64> = vec![160.0]; // initial value for moving average process
    println!("x1: {:?}\n", x1);
    
    let x2: Vec<f64> = vec![140.0]; // initial value for data generating process
    println!("x2: {:?}\n", x2);

    let c1 = VectorLWE::encode_encrypt(&context.sk, &x1, &enc)?;  
    println!("x1* {:?}", c1.decrypt_decode(&context.sk).unwrap());
    c1.pp();

    let c2 = VectorLWE::encode_encrypt(&context.sk, &x2, &enc)?;  
    println!("x2* {:?}",c2.decrypt_decode(&context.sk).unwrap());
    c2.pp();  
        
    let c3 = context.weighted_mean_of_pair(&c1, &c2, 0.5, &enc);
    println!("res* {:?}", c3.decrypt_decode(&context.sk).unwrap());
    c3.pp();   
    
    println!("res: {}", 0.5*&x1[0]+0.5*&x2[0]);

    
    // test mean_of_many
    
    let xv: Vec<f64> = vec![179.0,175.0,183.0,189.0,183.0,185.0,188.0,182.0,175.0,171.0];
    let n = xv.len();
    
    let cv = (0..n).map(
                        |i| VectorLWE::encode_encrypt(&context.sk, &[xv[i]], &enc).unwrap()
                    ).collect::<Vec<_>>();
    println!("cv[0]* {:?}", cv[0].decrypt_decode(&context.sk).unwrap());
    cv[0].pp(); 
    
    let avg = context.weighted_mean_of_many(&cv, &enc);
    println!("avg* {:?}", avg.decrypt_decode(&context.sk).unwrap());
    avg.pp(); 
    
    let tst = xv.iter().sum::<f64>() / xv.len() as f64;
    println!("tst: {}",tst);

    Ok(())
}
