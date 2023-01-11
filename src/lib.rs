use serde::*;
use concrete::*;
use concrete::{VectorLWE, LWE};
use rayon::prelude::*;

use std::fs::File;
use std::io::BufReader;
use std::cmp::{max, min};
// use std::ops::{Add, AddAssign};


/*
    KEY CREATION, SAVING, LOADING
*/

/*
#### ---- CREATE NEW SECRET KEY ---- ####
Input: ( Dimension, Noise level )
Output: ( Secret key )
*/
pub fn new_key(dimension: usize, noise_level: i32) -> LWESecretKey {

    let lwe_params: LWEParams = LWEParams::new(dimension, noise_level);
    let key = LWESecretKey::new(&lwe_params);

    return key
}

/*
#### ---- CREATE NEW (PROGRAMABLE) BOOTSTRAPPING KEY ---- ####
Input: ( IN KEY, (Optional) OUT KEY )
Output: ( Bootstrap key )
*/
pub fn new_bsk(in_key: &LWESecretKey, out_key: Option<&LWESecretKey>) -> LWEBSK {

    let bsk_key = match out_key {
        Some(k) => create_bsk_two_keys(in_key, k),
        None => create_bsk_one_keys(in_key),
    };

    return bsk_key;
}

/*
#### ---- CREATE BOOTSTRAPPING KEY, In key =/= Out key ---- ####
Input: ( IN KEY, OUT KEY )
Output: ( Bootstrap key )
*/
fn create_bsk_two_keys(in_key: &LWESecretKey, out_key: &LWESecretKey) -> LWEBSK{
    
    let rlwe_key = out_key.to_rlwe_secret_key(out_key.dimension).unwrap(); 

    let bsk = LWEBSK::new(in_key, &rlwe_key, 7, 7);

    return bsk;
}


/*
#### ---- CREATE BOOTSTRAPPING KEY, In key = Out key ---- ####
Input: ( IN KEY )
Output: ( Bootstrap key )
*/
fn create_bsk_one_keys(in_key: &LWESecretKey) -> LWEBSK{

    let rlwe_key = in_key.to_rlwe_secret_key(in_key.dimension).unwrap(); 

    let bsk = LWEBSK::new(in_key, &rlwe_key, 7, 7);

    return bsk;
}

/*
#### ---- CREATE NEW KEYSWITCHING KEY ---- ####
Input: ( IN KEY, (Optional) OUT KEY )
Output: ( Keyswitching key )
*/
pub fn new_ksk(in_key: &LWESecretKey, out_key: &LWESecretKey) -> LWEKSK {

    let ksk = LWEKSK::new(in_key, out_key, 6, 6);

    return ksk;
}

/*
#### ---- SAVING KEYS ---- ####
Input: ( KEYIDENTITY: ID )
Output: ( *No output )
*/
pub trait Save {
    fn save_key(&self, id: &str) -> ();
}
impl Save for LWESecretKey{
    fn save_key(&self, id: &str) -> (){
        self.save(&format!("{}{}{}", "keys/", "LWE", id));
    }
}
impl Save for LWEBSK{
    fn save_key(&self, id: &str) -> (){
        self.save(&format!("{}{}{}", "keys/", "BSK", id));
    }
}
impl Save for LWEKSK{
    fn save_key(&self, id: &str) -> (){
        self.save(&format!("{}{}{}", "keys/", "KSK", id));
    }
}

/*
#### ---- LOADING KEYS ---- ####
Input: ( KEYIDENTITY: ID )
Output: ( KEY )
*/
pub fn load_sk(id: &str) -> LWESecretKey {
    return LWESecretKey::load(&format!("{}{}{}", "keys/", "LWE", id)).unwrap();
}

pub fn load_bsk(id: &str) -> LWEBSK {
    return LWEBSK::load(&format!("{}{}{}", "keys/", "BSK", id))
}

pub fn load_ksk(id: &str) -> LWEKSK {
    return LWEKSK::load(&format!("{}{}{}", "keys/", "KSK", id))
}




/*
Works to print names of objects
SHOULD NOT BE USED OUTSIDE OF DEBUGGING
*/
// pub fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

/*
Trait that gives you the name of structures, IF you already are expecting them
 */
pub trait TypeInfo {
    fn type_of(&self) -> &'static str;
}
impl TypeInfo for LWESecretKey {
    fn type_of(&self) -> &'static str {
        "LWESecretKey"
    }
}





/* 
    VECTOR MANIPULATIONS
*/
fn sum_ct_VectorLWE(mut c: VectorLWE, new_min: f64) -> VectorLWE{
    let lenght = c.nb_ciphertexts;
    let mut ct_min = 0.;
    let mut min = 0.;
    let mut ct_min_arr = vec![0.; lenght];
    
    for i in 0..lenght{
        min = f64::abs(f64::min(0., c.encoders[i].get_min() as f64));
        ct_min += min;
        ct_min_arr[i] = min;
    }
    
    c.add_constant_static_encoder_inplace(&ct_min_arr).unwrap();
    let mut ct = c.sum_with_new_min(ct_min+new_min).unwrap();
    ct.add_constant_dynamic_encoder_inplace(&[-1.*ct_min]).unwrap();
    
    return ct;
}

// fn sum_N_VectorLWE(x: &VectorLWE) -> VectorLWE{
//     let mut y = x.clone();
//     let mut number = x.nb_ciphertexts as f64;
//     let mut n = 0;
//     while number/2. == f64::floor(number/2.){
//         n += 1;
//         number /= 2.;
//     }
//     let padd = x.encoders[0].nb_bit_padding;
//     let mut ct_1: VectorLWE;
//     let mut ct_2: VectorLWE;
    

//     for i in 0..(n as usize){
//         //y.pp();
//         if ((padd as i32) - (n as i32) <= 0) && (y.encoders[0].nb_bit_padding == 1){
//             //println!("Not enough padding!");
//             return y;
//         }else{
//             let N = u32::pow(2, (n-i-1) as u32) as usize;
//             let mut tmpVec = VectorLWE::zero(x.dimension, N).unwrap();
//             for j in 0..N{
//                 ct_1 = y.extract_nth(2*j).unwrap();
//                 ct_2 = y.extract_nth(2*j+1).unwrap();

//                 ct_1.add_with_padding_inplace(&ct_2).unwrap();

//                 tmpVec.copy_in_nth_nth_inplace(j, &ct_1, 0).unwrap();
//             }
//             y = tmpVec.clone();
//             //y.pp();
//         }
//     }
//     if y.nb_ciphertexts > 1{
//         y = sum_ct_VectorLWE(y, 0.);
//     }
//     return y;    
// }

/*
fn sum_ct_LWE(mut c: Vec<LWE>, new_min: f64) -> Vec<LWE>{
    let lenght = c.len();
    let mut ct_min = 0.;
    let mut min = 0.;
    let mut ct_min_arr = vec![0.; lenght];
    
    for i in 0..lenght{
        min = f64::abs(f64::min(0., c[i].encoder.get_min() as f64));
        ct_min += min;
        ct_min_arr[i] = min;
    }
    
    c.add_constant_static_encoder_inplace(&ct_min_arr).unwrap();
    let mut ct = c.sum_with_new_min(ct_min+new_min).unwrap();
    ct.add_constant_dynamic_encoder_inplace(&[-1.*ct_min]).unwrap();
    
    return ct;
}*/

fn sum_N_LWE(x: &Vec<LWE>) -> Vec<LWE>{
    let mut y = x.clone();
    let mut number = x.len() as f64;
    let mut n = 0;
    while number/2. == f64::floor(number/2.){
        n += 1;
        number /= 2.;
    }
    let padd = x[0].encoder.nb_bit_padding;
    let mut ct_1: LWE;
    let mut ct_2: LWE;
    

    for i in 0..(n as usize){
        //y.pp();
        if ((padd as i32) - (n as i32) <= 0) && (y[0].encoder.nb_bit_padding == 1){
            //println!("Not enough padding!");
            return y;
        }else{
            let N = u32::pow(2, (n-i-1) as u32) as usize;
            let mut tmpVec = vec![x[0].clone(); N];
            for j in 0..N{
                ct_1 = y[2*j].clone();
                ct_2 = y[2*j+1].clone();

                ct_1.add_with_padding_inplace(&ct_2).unwrap();

                tmpVec[j] = ct_1;
            }
            y = tmpVec.clone();
        }
    }
    //Vec<LWE> got no "sum_with_new_min"
    /*if y.len() > 1{
        y = sum_ct_VectorLWE(y, 0.);
    }*/
    return y;
}

pub fn VecLWE_to_ListVecLWE(ct: &VectorLWE) -> Vec<VectorLWE>{

    let len = ct.nb_ciphertexts;

    let mut list = vec![];
    for i in 0..len{
        list.push(ct.extract_nth(i).unwrap());
    }
    
    return list;
}

pub fn ListVecLWE_to_VecLWE(ct: Vec<VectorLWE>) -> VectorLWE{
    
    let len =  ct.len();
    let dim = ct[0].dimension;
    let mut VecLWE = VectorLWE::zero(dim, len).unwrap();

    for i in 0..len{
        VecLWE.copy_in_nth_nth_inplace(i, &ct[i], 0).unwrap();
    }

    return VecLWE;    
}
pub trait Append {
    fn append(&self, other: Self, count: usize) -> Self;
}
impl Append for VectorLWE{
    fn append(&self, other: Self, count: usize) -> VectorLWE{
        let mut list = VecLWE_to_ListVecLWE(self);
        for _ in 0..count{
            list.push(other.clone());
        }
        let vec = ListVecLWE_to_VecLWE(list);
        return vec;
    }
}


pub trait Sum {
    fn sum(&self) -> Self;
}
impl Sum for VectorLWE{
    fn sum(&self) -> VectorLWE{
        let list_Vec = VecLWE_to_ListVecLWE(&self.clone());
        return ListVecLWE_to_VecLWE(list_Vec.sum());
    }
}
// impl Sum for Vec<LWE>{
//     fn sum(&self) -> Vec<LWE>{
//     }
// }
impl Sum for Vec<VectorLWE>{
    fn sum(&self) -> Vec<VectorLWE>{
        let padd_count = self[0].encoders[0].nb_bit_padding - 1;
        let div_by_2_count = count_divides(self.len());

        let padding_add_count = min(padd_count, div_by_2_count);

        // println!("amount of padding: {}", &padd_count);
        // println!("number of time vec is divisible by 2: {}", &div_by_2_count);

        // println!("min of these: {}\n", &padding_add_count);

        // self[0].pp();
        // println!("len: {}", &self.len());

        let res = self.clone().sum_padd(padding_add_count);
        // res[0].pp();
        // println!("len: {}", &res.len());

        let res = res.clone().sum_min();
        // res[0].pp();
        // println!("len: {}", &res.len());

        return res;
    }
}

pub trait Sum_padd {
    fn sum_padd(&self, count: usize) -> Self;
}
impl Sum_padd for Vec<VectorLWE>{
    fn sum_padd(&self, count: usize) -> Self{
        
        let mut z = self.clone();

        for _ in 0..count{
            let n = z.len();
            let x = z[0..n/2].to_vec();
            let y = z[n/2..n].to_vec();

            z = VecLWE_to_ListVecLWE(&ListVecLWE_to_VecLWE(x).add_with_padding(&ListVecLWE_to_VecLWE(y)).unwrap());
        };

        return z;
    }
}

pub trait SumMin {
    fn sum_min(&self) -> Self;
}
impl SumMin for Vec<VectorLWE> {
    fn sum_min(&self) -> Self{
        let x = ListVecLWE_to_VecLWE(self.clone());
        let y = sum_ct_VectorLWE(x, 0.);
        return VecLWE_to_ListVecLWE(&y);
    }
}

fn count_divides(lenght: usize) -> usize{
    let n: i32 = 2;
    let mut i: u32 = 0;

    while (lenght as i32) % n.pow(i+1) == 0 {
        i += 1;
    }
    return i as usize;
}

// pub trait Add{
//     type Output;
//     fn add(self, other: Self) -> Self::Output;
// }
// impl Add for VectorLWE{
//     type Output = VectorLWE;
//     fn add(self, other: VectorLWE) -> VectorLWE{
//         return self.add_with_padding(&other).unwrap();
//     }
// }
// impl Add for Vec<VectorLWE>{
//     type Output = Vec<VectorLWE>;
//     // VecLWE_to_ListVecLWE();
//     // ListVecLWE_to_VecLWE();

//     fn add(self, other: Self) -> Vec<VectorLWE>{
//         let output = VecLWE_to_ListVecLWE(ListVecLWE_to_VecLWE(self) + ListVecLWE_to_VecLWE(other));
//     }
// }

// impl AddAssign for VectorLWE{
//     fn add_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         };
//     }
// }

/*
    VECTOR-LWE / LWE OPERATIONS
*/
pub trait CheckVar {
    fn check_vars(&self) -> Self;
}
impl CheckVar for VectorLWE {
    fn check_vars(&self) -> Self{

        let mut x = self.clone();
        
        for var in x.variances.iter_mut(){
            *var += 2.0_f64.powi(-83);
        }
        
        return x;
    }
}



/* 
    FUNCTIONS
*/
// n = 1: ordinary step function, n = -1: reverse step function
pub fn step_function(x: f64, a: f64, n: i32) -> f64{
    if n == 1{
        if x > a {
            return 1.0;
        }
        else {
            return 0.0;
        }
    } else{
        if x < a {
            return 1.0;
        }
        else {
            return 0.0;
        }
    }
}

fn relu(x: f64) -> f64{
    return f64::max(x, 0.);
}

fn elu_plus_one(x: f64) -> f64{
    if x >= 0. {
        return x+1.001;
    }
    else {
        return f64::exp(x)
    }
}





// ##### ----- SOME DEEP LEARNING STUFF ----- #####
pub trait LoadData {
    fn load(path: &str) -> (Vec<Vec<f64>>, Vec<Vec<f64>>);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data{
    test_datas: Vec<Vec<f64>>,
    test_labels: Vec<Vec<f64>>,
}
impl LoadData for Data {
    fn load(path: &str) -> (Vec<Vec<f64>>, Vec<Vec<f64>>){
        println!("Loading Data!\n");
        let file = File::open("data/".to_owned()+path+".json").unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file
        let data: Data = serde_json::from_reader(reader).unwrap();

        return (data.test_datas, data.test_labels);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pred{
    mus: Vec<Vec<f64>>,
    sigs: Vec<Vec<f64>>,
}
impl LoadData for Pred {
    fn load(path: &str) -> (Vec<Vec<f64>>, Vec<Vec<f64>>){
        println!("Loading Data!\n");
        let file = File::open("data/".to_owned()+path+".json").unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file
        let data: Pred = serde_json::from_reader(reader).unwrap();

        return (data.mus, data.sigs);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer{
    weights: Vec<Vec<f64>>,
    bias: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model{
    Layer_0: Layer,
    Layer_1: Layer,
    Layer_2: Layer,
    Layer_mu: Layer,
    Layer_sig: Layer,
}

#[derive(Debug)]
pub struct Net{
    Bit_precision: usize,
    Encoder: Encoder,
    bsk: LWEBSK,
    ksk: LWEKSK,
    pub zero: Option<VectorLWE>,

    Model: Model,
    Hidden_size: usize,
    Output_size: usize,
}
impl Net{

    pub fn load_model(model_name: &str, id: &str) -> Net{

        println!("Loading Keys!\n");
        let bsk = load_bsk(id);
        let ksk = load_ksk(id);

        println!("Loading Model!\n");
        let file = File::open("models/".to_owned()+model_name+".json").unwrap();
        let reader = BufReader::new(file);
        // Read the JSON contents of the file
        let model: Model = serde_json::from_reader(reader).unwrap();
        
        
        let prec = 5;
        // let hidden_size = model.Layer_0.bias.len();
        // let output_size = model.Layer_mu.bias.len();

        let net = Net{
            Bit_precision: prec,
            Encoder: Encoder::new(0., 10., prec, prec+3+1).unwrap(),
            bsk: bsk,
            ksk: ksk,
            zero: None,

            Hidden_size: model.Layer_0.bias.len(),
            Output_size: model.Layer_mu.bias.len(),
            Model: model,
        };

        // println!{"{:?}", net.Model.Layer_0};

        return net;
    }

    // pub fn forward(&self, input: VectorLWE, bsk: &LWEBSK) -> (VectorLWE, VectorLWE){
    pub fn forward(&self, input: VectorLWE) -> (VectorLWE, VectorLWE){

        // let bit_precision = input.encoders[0].nb_bit_precision;
        // let enc_out = Encoder::new(0., 5., bit_precision, bit_precision+3).unwrap();
        // let self.Encoder = Encoder::new(0., 5., self.Bit_precision, self.Bit_precision+3).unwrap();

        let mut output_0 = VectorLWE::zero(input.dimension, self.Hidden_size).unwrap();
        let mut output_1 = VectorLWE::zero(input.dimension, self.Hidden_size).unwrap();
        let mut output_2 = VectorLWE::zero(input.dimension, self.Hidden_size).unwrap();
        let mut output_mu = VectorLWE::zero(input.dimension, self.Output_size).unwrap();
        let mut output_sig = VectorLWE::zero(input.dimension, self.Output_size).unwrap();

        // input.pp();
        println!("Layer 0!\n");
        for (i, weights) in self.Model.Layer_0.weights.iter().enumerate(){
            // println!("{}", weights.len());
            // println!("{}", self.Model.Layer_0.weights.len());
            // println!("{}", input.nb_ciphertexts);
            let mut ct_tmp = input.mul_constant_with_padding(weights, 1.5, self.Bit_precision).unwrap();
            ct_tmp.pp();
            // ct_tmp = sum_N_VectorLWE(&ct_tmp);
            // println!("{}", ct_tmp.nb_ciphertexts);
            // let minVar = ct_tmp.variances.iter().min();
            // match minVar {
            //     Some(min) => println!( "Min value: {}", min ),
            //     None      => println!( "Vector is empty" ),
            // }
            ct_tmp = ct_tmp.check_vars();
            // let minVar = &ct_tmp.variances.clone().into_iter().min_by(|a, b| a.partial_cmp(b).unwrap());
            // println!("{}", minVar.unwrap().log2());

            ct_tmp = ct_tmp.append(self.zero.clone().unwrap().mul_constant_with_padding(&vec![0.], 1.5, self.Bit_precision).unwrap(), 32-19);
            // println!("{}", ct_tmp.nb_ciphertexts);
            // ct_tmp.pp();
            ct_tmp = ct_tmp.clone().sum();
            // println!("{}", ct_tmp.nb_ciphertexts);
            ct_tmp.pp();
            //add bias

            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap();
            ct_tmp2.pp();
            output_0.copy_in_nth_nth_inplace(i, &ct_tmp2, 0).unwrap();
            // output_0.copy_in_nth_nth_inplace(i, &(ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap()), 0).unwrap();
            // break;
            
        }

        ///output_1.pp();
        println!("Layer 1!\n");
        for (i, weights) in self.Model.Layer_1.weights.iter().enumerate(){
            // output_0.pp();
            let mut ct_tmp = output_0.mul_constant_with_padding(weights, 1.5, self.Bit_precision).unwrap();
            // println!("{}", ct_tmp.nb_ciphertexts);
            // ct_tmp.pp();
            // ct_tmp = sum_N_VectorLWE(&ct_tmp);

            // ct_tmp.add_constant_static_encoder_inplace(&vec![0.0; ct_tmp.nb_ciphertexts]);
            // ct_tmp.pp();
            ct_tmp = ct_tmp.check_vars();
            // ct_tmp.pp();

            ct_tmp = ct_tmp.clone().sum();
            // println!("{}", ct_tmp.nb_ciphertexts);
            // ct_tmp.pp();
            //add bias
            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap();
            output_1.copy_in_nth_nth_inplace(i, &ct_tmp2, 0).unwrap();
            // output_1.copy_in_nth_nth_inplace(i, &(ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap()), 0).unwrap();
            // break
        }

        ///output_1.pp();
        println!("Layer 2!\n");
        for (i, weights) in self.Model.Layer_2.weights.iter().enumerate(){
            let mut ct_tmp = output_1.mul_constant_with_padding(weights, 1.6, self.Bit_precision).unwrap();
            //ct_tmp.pp();
            // ct_tmp = sum_N_VectorLWE(&ct_tmp);
            ct_tmp = ct_tmp.check_vars();

            ct_tmp = ct_tmp.clone().sum();
            //ct_tmp.pp();
            //add bias
            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap();
            output_2.copy_in_nth_nth_inplace(i, &ct_tmp2, 0).unwrap();
            // output_2.copy_in_nth_nth_inplace(i, &(ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap()), 0).unwrap();
        }

        println!("Layer mu!\n");
        for (i, weights) in self.Model.Layer_mu.weights.iter().enumerate(){
            let mut ct_tmp = output_2.mul_constant_with_padding(weights, 1.5, self.Bit_precision).unwrap();
            // ct_tmp = sum_N_VectorLWE(&ct_tmp);
            ct_tmp = ct_tmp.check_vars();

            ct_tmp = ct_tmp.clone().sum();
            //ct_tmp.pp();
            //add bias
            output_mu.copy_in_nth_nth_inplace(i, &ct_tmp, 0).unwrap();
        }

        println!("Layer sig!\n");
        for (i, weights) in self.Model.Layer_sig.weights.iter().enumerate(){
            let mut ct_tmp = output_2.mul_constant_with_padding(weights, 1.5, self.Bit_precision).unwrap();
            // ct_tmp = sum_N_VectorLWE(&ct_tmp);
            ct_tmp = ct_tmp.check_vars();

            ct_tmp = ct_tmp.clone().sum();
            //ct_tmp.pp();
            //add bias
            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap();
            output_sig.copy_in_nth_nth_inplace(i, &ct_tmp2, 0).unwrap();
            // output_sig.copy_in_nth_nth_inplace(i, &(ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| elu_plus_one(x), &self.Encoder, 0).unwrap()), 0).unwrap();
        }
        
        return (output_mu, output_sig);
    }

    // fn calculate_layer(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>, func: fn(f64) -> f64, bsk: &LWEBSK, enc: &Encoder, prec: usize) -> Vec<LWE> {        
        //     let mut output = vec![input[0].clone(); layer_weights.len()];
    
        //     output.par_iter_mut().zip(layer_weights.par_iter()).for_each(| (out, weights) |{
                
        //         let mut layer_tmp = input.clone();
        //         weights.par_iter().zip(input.par_iter().zip(layer_tmp.par_iter_mut())).for_each(| (w, (ct_i, ct_o)) |{
        //             *ct_o = ct_i.mul_constant_with_padding(*w, 1., self.Bit_precision.clone()).unwrap();
        //         });
    
        //         let ct_tmp = &sum_N_LWE(&layer_tmp)[0];
        //         *out = ct_tmp.bootstrap_with_function(&bsk, |x| func(x), &self.Encoder).unwrap();
        //     });
    
        //     return output;
        // }
    // fn calculate_layer(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>, func: fn(f64) -> f64, bsk: &LWEBSK) -> Vec<LWE> {
    fn calculate_layer(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>, func: fn(f64) -> f64) -> Vec<LWE> {
        
        let mut output = vec![input[0].clone(); layer_weights.len()];

        output.par_iter_mut().zip(layer_weights.par_iter()).for_each(| (out, weights) |{
            
            let mut layer_tmp = input.clone();
            weights.par_iter().zip(input.par_iter().zip(layer_tmp.par_iter_mut())).for_each(| (w, (ct_i, ct_o)) |{
                *ct_o = ct_i.mul_constant_with_padding(*w, 1., self.Bit_precision.clone()).unwrap();
            });

            let ct_tmp = &sum_N_LWE(&layer_tmp)[0];
            // *out = ct_tmp.bootstrap_with_function(&bsk, |x| func(x), &self.Encoder).unwrap();
            *out = ct_tmp.bootstrap_with_function(&self.bsk, |x| func(x), &self.Encoder).unwrap();
        });

        return output;
    }

    // fn calculate_layer_nofunc(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>, prec: usize) -> Vec<LWE> {
    //     let mut output = vec![input[0].clone(); layer_weights.len()];

    //     output.par_iter_mut().zip(layer_weights.par_iter()).for_each(| (out, weights) |{
            
    //         let mut layer_tmp = input.clone();
    //         weights.par_iter().zip(input.par_iter().zip(layer_tmp.par_iter_mut())).for_each(| (w, (ct_i, ct_o)) |{
    //             *ct_o = ct_i.mul_constant_with_padding(*w, 1., self.Bit_precision.clone()).unwrap();
    //         });

    //         //let ct_tmp = sum_N_LWE(&layer_tmp);
    //         *out = sum_N_LWE(&layer_tmp)[0].clone();//.bootstrap_with_function(&bsk, |x| func(x), &enc).unwrap();
    //     });

    //     return output;
    // }

    fn calculate_layer_nofunc(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>) -> Vec<LWE> {
        
        let mut output = vec![input[0].clone(); layer_weights.len()];

        output.par_iter_mut().zip(layer_weights.par_iter()).for_each(| (out, weights) |{
            
            let mut layer_tmp = input.clone();
            weights.par_iter().zip(input.par_iter().zip(layer_tmp.par_iter_mut())).for_each(| (w, (ct_i, ct_o)) |{
                *ct_o = ct_i.mul_constant_with_padding(*w, 1., self.Bit_precision.clone()).unwrap();
            });

            //let ct_tmp = sum_N_LWE(&layer_tmp);
            *out = sum_N_LWE(&layer_tmp)[0].clone();//.bootstrap_with_function(&bsk, |x| func(x), &enc).unwrap();
        });

        return output;
    }

    // pub fn forward_par(&self, mut input: Vec<LWE>, bsk: &LWEBSK) -> (Vec<LWE>, Vec<LWE>){
    pub fn forward_par(&self, input: Vec<LWE>) -> (Vec<LWE>, Vec<LWE>){

        // let bit_precision = input[0].encoder.nb_bit_precision;
        // let enc_out = Encoder::new(0., 5., bit_precision, bit_precision+3).unwrap();

        let ct_layer_0_o = self.calculate_layer(input, &self.Model.Layer_0.weights, relu);

        let ct_layer_1_o = self.calculate_layer(ct_layer_0_o, &self.Model.Layer_1.weights, relu);

        let ct_layer_2_o = self.calculate_layer(ct_layer_1_o, &self.Model.Layer_2.weights, relu);

        let output_mu = self.calculate_layer_nofunc(ct_layer_2_o.clone(), &self.Model.Layer_mu.weights);

        let output_sig = self.calculate_layer(ct_layer_2_o.clone(), &self.Model.Layer_sig.weights, elu_plus_one);

        return (output_mu, output_sig);
    }
}
