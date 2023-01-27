use serde::*;
use concrete::*;
use concrete::{VectorLWE, LWE};
use rayon::prelude::*;

use std::fs::File;
use std::io::BufReader;
use std::cmp::{max, min};
use std::ops::{Add, AddAssign};


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
    
    println!("dual-key bsk");
    let rlwe_key = out_key.to_rlwe_secret_key(out_key.dimension).unwrap(); 

    let bsk = LWEBSK::new(in_key, &rlwe_key, 6, 9);

    return bsk;
}


/*
#### ---- CREATE BOOTSTRAPPING KEY, In key = Out key ---- ####
Input: ( IN KEY )
Output: ( Bootstrap key )
*/
fn create_bsk_one_keys(in_key: &LWESecretKey) -> LWEBSK{

    println!("singel-key bsk");

    let rlwe_key = in_key.to_rlwe_secret_key(in_key.dimension).unwrap(); 

    let bsk = LWEBSK::new(in_key, &rlwe_key, 6, 8);

    return bsk;
}

/*
#### ---- CREATE NEW KEYSWITCHING KEY ---- ####
Input: ( IN KEY, (Optional) OUT KEY )
Output: ( Keyswitching key )
*/
pub fn new_ksk(in_key: &LWESecretKey, out_key: &LWESecretKey) -> LWEKSK {

    let ksk = LWEKSK::new(in_key, out_key, 6, 6);

    println!("before: {}, after: {}", ksk.dimension_before, ksk.dimension_after);

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

// pub struct Vec<VectorLWE>{
//     list: Vec<VectorLWE>,
// }

fn sum_ct_VectorLWE(mut c: VectorLWE, new_min: f64, sk: &LWESecretKey) -> VectorLWE{
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
    // fn sum(&self) -> Self;
    fn sum(&self, sk: &LWESecretKey) -> Self;
}
impl Sum for VectorLWE{
    // fn sum(&self) -> VectorLWE{
    fn sum(&self, sk: &LWESecretKey) -> VectorLWE{
        let list_Vec = VecLWE_to_ListVecLWE(&self.clone());
        return ListVecLWE_to_VecLWE(list_Vec.sum(sk));
    }
}
// impl Sum for Vec<LWE>{
//     fn sum(&self) -> Vec<LWE>{
//     }
// }
impl Sum for Vec<VectorLWE>{
    // fn sum(&self) -> Vec<VectorLWE>{
    fn sum(&self, sk: &LWESecretKey) -> Vec<VectorLWE>{
        let padd_count = self[0].encoders[0].nb_bit_padding - 1;
        let div_by_2_count = count_divides(self.len());

        let padding_add_count = min(padd_count, div_by_2_count);

        // println!("amount of padding: {}", &padd_count);
        // println!("number of time vec is divisible by 2: {}", &div_by_2_count);

        // println!("min of these: {}\n", &padding_add_count);

        // self[0].pp();
        // println!("len: {}", &self.len());

        let mut sum = 0.;
        for d in (ListVecLWE_to_VecLWE(self.clone()).decrypt_decode(sk).unwrap()).iter(){
            sum += d;
        }
        // println!("1 sum: {}", sum);

        let res = self.clone().sum_padd(padding_add_count);

        let mut sum = 0.;
        for d in (ListVecLWE_to_VecLWE(res.clone()).decrypt_decode(sk).unwrap()).iter(){
            sum += d;
        }
        // println!("2 sum: {}", sum);

        // res[0].pp();
        // println!("len: {}", &res.len());

        let res = res.clone().sum_min(sk);

        let mut sum = 0.;
        for d in (ListVecLWE_to_VecLWE(res.clone()).decrypt_decode(sk).unwrap()).iter(){
            sum += d;
        }
        // println!("3 sum: {}\n", sum);
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
    // fn sum_min(&self) -> Self;
    fn sum_min(&self, sk: &LWESecretKey) -> Self;
}
impl SumMin for Vec<VectorLWE> {
    // fn sum_min(&self) -> Self{
    fn sum_min(&self, sk: &LWESecretKey) -> Self{
        let x = ListVecLWE_to_VecLWE(self.clone());

        // println!("{:?}", &x.encoders);
        // // let y = sum_ct_VectorLWE(x, 0., sk);
        // println!("{:?}", &x.decrypt_decode(&sk).unwrap());

        let minimum = x.encoders.clone().into_iter().min_by(|a, b| a.o.partial_cmp(&b.o).unwrap()).unwrap().o;
        // println!("min: {}", minimum);

        let delta = x.encoders[0].delta;
        // println!("in, delta: {}", delta);

        let prec: i32 = (x.encoders.clone().into_iter().map(|s| (s.nb_bit_precision as f64)/(x.nb_ciphertexts as f64)).sum::<f64>() as f64).round() as i32;
        // println!("in, prec: {}", prec);

        let range = delta * (f64::powi(2., prec) - 1.)/ (f64::powi(2., prec));
        // println!("in, range: {}", range);

        let center: f64  = x.encoders.clone().into_iter().map(|s| (2.*s.o + range)/2.).sum();
        // println!("in, center: {}", center);


        let potential_min = center - range/2.;
        // println!("pot_min: {}", potential_min);

        let new_min = f64::min(minimum, potential_min);

        let output = x.sum_with_new_min(new_min).unwrap();//.decrypt_decode(&sk).unwrap());

    return VecLWE_to_ListVecLWE(&output);

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

// pub unsafe trait Add{
//     type Output;
//     fn add(self, other: Self) -> Self::Output;
// }
// unsafe impl Add for VectorLWE{
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
            *var += 2.0_f64.powi(-130);
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

fn max_relu(x: f64, a: f64) -> f64{
    return f64::min(f64::max(x, 0.), a);
}

fn elu_plus_one(x: f64) -> f64{
    if x > 0. {
        return x+1.0;
    }
    else {
        return f64::exp(x)
    }
}

fn elu_plus_one_max(x: f64, a: f64) -> f64{
    if x > 0. {
        return f64::min(x+1.0, a);
    }
    else {
        return f64::min(f64::exp(x), a);
    }
}

fn sigmoid(x: f64) -> f64{
    return 1.0/(1.0 + (-x).exp());
}

fn sigmoid_scaled(x: f64, a: f64) -> f64{
    return 1.0/(1.0 + (-x*a).exp());
}

fn tanh(x: f64) -> f64{
    return x.tanh();
}

fn tanh_scaled(x: f64, a: f64) -> f64{
    return (x/a).tanh();
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
    max_const: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model{
    Layer_0: Layer,
    Layer_1: Layer,
    // Layer_2: Layer,
    Layer_mu: Layer,
    Layer_sig: Layer,
}

#[derive(Debug)]
pub struct Net{
    Bit_precision: usize,
    Encoder_0: Encoder,
    Encoder_1: Encoder,
    Encoder_2: Encoder,
    Encoder_mu: Encoder,
    Encoder_sig: Encoder,
    sk: LWESecretKey,
    sk_out: LWESecretKey,
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
        let sk = load_sk(id);
        let sk_out = load_sk(&(id.to_owned()+"_out"));
        let bsk = load_bsk(id);
        let ksk = load_ksk(id);

        println!("Loading Model!\n");
        let file = File::open("models/".to_owned()+model_name+".json").unwrap();
        let reader = BufReader::new(file);
        // Read the JSON contents of the file
        let model: Model = serde_json::from_reader(reader).unwrap();
        
        
        let prec = 6;
        // let hidden_size = model.Layer_0.bias.len();
        // let output_size = model.Layer_mu.bias.len();

        let net = Net{
            Bit_precision: prec,
            Encoder_0: Encoder::new(0., 2., prec, prec+3+1).unwrap(),
            Encoder_1: Encoder::new(0., 2., prec, prec+3+1).unwrap(),
            Encoder_2: Encoder::new(0., 2., prec, prec+3+1).unwrap(),
            Encoder_mu: Encoder::new(-1., 1., prec, 0).unwrap(),
            Encoder_sig: Encoder::new(0., 1.5, prec, 0).unwrap(),
            sk: sk,
            sk_out: sk_out, 
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
        let scale_0 = self.Model.Layer_0.max_const;
        for (i, (weights, bias)) in self.Model.Layer_0.weights.iter().zip(self.Model.Layer_0.bias.iter()).enumerate(){

            let mut ct_tmp = input.mul_constant_with_padding(weights, scale_0, self.Bit_precision).unwrap();

            ct_tmp = ct_tmp.check_vars();
            
            let mut sum = 0.;
            for d in (ct_tmp.decrypt_decode(&self.sk).unwrap()).iter(){
                sum += d;
            }
            sum += bias.clone();

            println!("plain sum: {}, func = {}", sum, elu_plus_one_max(sum, 2.));

            ct_tmp = ct_tmp.clone().sum(&self.sk);
            // ct_tmp.add_constant_static_encoder_inplace(&[*bias]).unwrap();
            ct_tmp.add_constant_dynamic_encoder_inplace(&[*bias]).unwrap();

            // println!("enc sum ( input * weights ) = {:?}, func = {}", ct_tmp.decrypt_decode(&self.sk).unwrap(), elu_plus_one_max(ct_tmp.decrypt_decode(&self.sk).unwrap()[0], 2.0));
            // println!("enc sum ( input * weights ) = {:?}, max_relu= {}", ct_tmp.decrypt_decode(&self.sk).unwrap(), max_relu(ct_tmp.decrypt_decode(&self.sk).unwrap()[0], 2.));
            // println!("enc sum ( input * weights ) = {:?}, max_relu= {}", ct_tmp.decrypt_decode(&self.sk_out).unwrap(), max_relu(ct_tmp.decrypt_decode(&self.sk_out).unwrap()[0], 2.));
            
            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| elu_plus_one_max(x, 2.), &self.Encoder_0, 0).unwrap();
            // let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| sigmoid(x)*2.0, &self.Encoder_0, 0).unwrap();
            // let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| max_relu(x, 2.), &self.Encoder_0, 0).unwrap();
            // let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap();
            // println!("func ( enc sum ( input * weights ) ) = {:?}\n\n", ct_tmp2.decrypt_decode(&self.sk_out).unwrap());
            
            let ct_tmp3 = ct_tmp2.keyswitch(&self.ksk).unwrap();

             output_0.copy_in_nth_nth_inplace(i, &ct_tmp3, 0).unwrap();
            
        }

        println!("Layer 1!\n");
        for (i, (weights, bias)) in self.Model.Layer_1.weights.iter().zip(self.Model.Layer_1.bias.iter()).enumerate(){
            
            let mut ct_tmp = output_0.mul_constant_with_padding(weights, self.Model.Layer_1.max_const, self.Bit_precision).unwrap();
            
            ct_tmp = ct_tmp.check_vars();

            
            let mut sum = 0.;
            for d in (ct_tmp.decrypt_decode(&self.sk).unwrap()).iter(){
                sum += d;
            }
            sum += bias.clone();

            // println!("plain sum: {}, max_relu= {}", sum, max_relu(sum, 2.));
            // println!("plain sum: {}, func = {}", sum, elu_plus_one_max(sum, 2.));

            ct_tmp = ct_tmp.clone().sum(&self.sk);

            ct_tmp.add_constant_dynamic_encoder_inplace(&[*bias]).unwrap();

            // println!("enc sum ( input * weights ) = {:?}, max_relu= {}", ct_tmp.decrypt_decode(&self.sk).unwrap(), max_relu(ct_tmp.decrypt_decode(&self.sk).unwrap()[0], 2.));
            // println!("enc sum ( input * weights ) = {:?}, func = {}", ct_tmp.decrypt_decode(&self.sk).unwrap(), elu_plus_one_max(ct_tmp.decrypt_decode(&self.sk).unwrap()[0], 2.));

            // let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| max_relu(x, 2.), &self.Encoder_1, 0).unwrap();
            // println!("max_relu ( enc sum ( input * weights ) ) = {:?}\n\n", ct_tmp2.decrypt_decode(&self.sk_out).unwrap());
            
            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| elu_plus_one_max(x, 2.), &self.Encoder_1, 0).unwrap();
            // let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| sigmoid(x)*2., &self.Encoder_1, 0).unwrap();
            // println!("func ( enc sum ( input * weights ) ) = {:?}\n\n", ct_tmp2.decrypt_decode(&self.sk_out).unwrap());
            
            output_1.copy_in_nth_nth_inplace(i, &ct_tmp2.keyswitch(&self.ksk).unwrap(), 0).unwrap();
        }

        /*
        ///output_1.pp();
        println!("Layer 2!\n");
        for (i, (weights, bias)) in self.Model.Layer_2.weights.iter().zip(self.Model.Layer_2.bias.iter()).enumerate(){
            // println!("encoder = {:?}", output_1.encoders[0]);
            let mut ct_tmp = output_1.mul_constant_with_padding(weights, self.Model.Layer_2.max_const, self.Bit_precision).unwrap();
            // println!("encoder = {:?}", ct_tmp.encoders[0]);

            //ct_tmp.pp();
            // ct_tmp = sum_N_VectorLWE(&ct_tmp);
            ct_tmp = ct_tmp.check_vars();

            let mut sum = 0.;
            for d in (ct_tmp.decrypt_decode(&self.sk).unwrap()).iter(){
                sum += d;
            }
            sum += bias.clone();
            // println!("plain sum: {}, sigmoid= {}", sum, sigmoid_scaled(sum, 10.));
            println!("plain sum: {}, max_relu= {}", sum, max_relu(sum, 5.));
            ct_tmp = ct_tmp.clone().sum(&self.sk);
            // ct_tmp.add_constant_static_encoder_inplace(&[*bias]).unwrap();
            ct_tmp.add_constant_dynamic_encoder_inplace(&[*bias]).unwrap();
            println!("enc sum ( input * weights ) = {:?}, max_relu= {}", ct_tmp.decrypt_decode(&self.sk).unwrap(), max_relu(ct_tmp.decrypt_decode(&self.sk).unwrap()[0], 5.));
            //ct_tmp.pp();
            //add bias
            // let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| sigmoid_scaled(x, 10.), &self.Encoder, 0).unwrap();
            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| max_relu(x, 5.), &self.Encoder_2, 0).unwrap();
            println!("max_relu ( enc sum ( input * weights ) ) = {:?}\n\n", ct_tmp2.decrypt_decode(&self.sk).unwrap());
            // let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap();
            output_2.copy_in_nth_nth_inplace(i, &ct_tmp2, 0).unwrap();
            // output_2.copy_in_nth_nth_inplace(i, &(ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| relu(x), &self.Encoder, 0).unwrap()), 0).unwrap();
        }
        */
        println!("Layer mu!\n");
        for (i, (weights, bias)) in self.Model.Layer_mu.weights.iter().zip(self.Model.Layer_mu.bias.iter()).enumerate(){

            let mut ct_tmp = output_1.mul_constant_with_padding(weights, self.Model.Layer_mu.max_const, self.Bit_precision).unwrap();

            ct_tmp = ct_tmp.check_vars();

            let mut sum = 0.;
            for d in (ct_tmp.decrypt_decode(&self.sk).unwrap()).iter(){
                sum += d;
            }
            sum += bias.clone();

            // println!("plain sum: {}", sum);
            ct_tmp = ct_tmp.clone().sum(&self.sk);

            ct_tmp.add_constant_dynamic_encoder_inplace(&[*bias]).unwrap();
            // println!("enc sum ( input * weights ) = {:?}, func = {}", ct_tmp.clone().decrypt_decode(&self.sk).unwrap(), tanh_scaled(ct_tmp.clone().decrypt_decode(&self.sk).unwrap()[0], 1.0));

            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x|  tanh_scaled(x, 1.0), &self.Encoder_mu, 0).unwrap();

            // println!("max_relu ( enc sum ( input * weights ) ) = {:?}\n\n", ct_tmp2.decrypt_decode(&self.sk_out).unwrap());
            // println!("func ( enc sum ( input * weights ) ) = {:?}\n\n", ct_tmp2.decrypt_decode(&self.sk_out).unwrap());

            output_mu.copy_in_nth_nth_inplace(i, &(ct_tmp2.keyswitch(&self.ksk).unwrap()), 0).unwrap();
        }

        println!("Layer sig!\n");
        for (i, (weights, bias)) in self.Model.Layer_sig.weights.iter().zip(self.Model.Layer_sig.bias.iter()).enumerate(){

            let mut ct_tmp = output_1.mul_constant_with_padding(weights, self.Model.Layer_sig.max_const, self.Bit_precision).unwrap();

            ct_tmp = ct_tmp.check_vars();

            let mut sum = 0.;
            for d in (ct_tmp.decrypt_decode(&self.sk).unwrap()).iter(){
                sum += d;
            }
            sum += bias.clone();

            // println!("plain sum: {}, func = {}", sum, elu_plus_one_max(sum, 1.5));
            ct_tmp = ct_tmp.clone().sum(&self.sk);

            // ct_tmp.add_constant_static_encoder_inplace(&[*bias]).unwrap();
            ct_tmp.add_constant_dynamic_encoder_inplace(&[*bias]).unwrap();

            // println!("enc sum ( input * weights ) = {:?}, func = {}", ct_tmp.clone().decrypt_decode(&self.sk).unwrap(), elu_plus_one_max(ct_tmp.clone().decrypt_decode(&self.sk).unwrap()[0], 1.5));

            let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&self.bsk, |x| elu_plus_one_max(x, 1.5), &self.Encoder_sig, 0).unwrap();
            // println!("func ( enc sum ( input * weights ) ) = {:?}\n\n", ct_tmp2.decrypt_decode(&self.sk_out).unwrap());

            output_sig.copy_in_nth_nth_inplace(i, &(ct_tmp2.keyswitch(&self.ksk).unwrap()), 0).unwrap();
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
    // fn calculate_layer(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>, func: fn(f64) -> f64) -> Vec<LWE> {
        
    //     let mut output = vec![input[0].clone(); layer_weights.len()];

    //     output.par_iter_mut().zip(layer_weights.par_iter()).for_each(| (out, weights) |{
            
    //         let mut layer_tmp = input.clone();
    //         weights.par_iter().zip(input.par_iter().zip(layer_tmp.par_iter_mut())).for_each(| (w, (ct_i, ct_o)) |{
    //             *ct_o = ct_i.mul_constant_with_padding(*w, 1., self.Bit_precision.clone()).unwrap();
    //         });

    //         let ct_tmp = &sum_N_LWE(&layer_tmp)[0];
    //         // *out = ct_tmp.bootstrap_with_function(&bsk, |x| func(x), &self.Encoder).unwrap();
    //         *out = ct_tmp.bootstrap_with_function(&self.bsk, |x| func(x), &self.Encoder_0).unwrap();
    //     });

    //     return output;
    // }

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

    // fn calculate_layer_nofunc(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>) -> Vec<LWE> {
        
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

    // pub fn forward_par(&self, mut input: Vec<LWE>, bsk: &LWEBSK) -> (Vec<LWE>, Vec<LWE>){
    // pub fn forward_par(&self, input: Vec<LWE>) -> (Vec<LWE>, Vec<LWE>){

    //     // let bit_precision = input[0].encoder.nb_bit_precision;
    //     // let enc_out = Encoder::new(0., 5., bit_precision, bit_precision+3).unwrap();

    //     let ct_layer_0_o = self.calculate_layer(input, &self.Model.Layer_0.weights, relu);

    //     let ct_layer_1_o = self.calculate_layer(ct_layer_0_o, &self.Model.Layer_1.weights, relu);

    //     let ct_layer_2_o = self.calculate_layer(ct_layer_1_o, &self.Model.Layer_2.weights, relu);

    //     let output_mu = self.calculate_layer_nofunc(ct_layer_2_o.clone(), &self.Model.Layer_mu.weights);

    //     let output_sig = self.calculate_layer(ct_layer_2_o.clone(), &self.Model.Layer_sig.weights, elu_plus_one);

    //     return (output_mu, output_sig);
    // }
}
