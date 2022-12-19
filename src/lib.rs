use concrete::*;
// use concrete::lwe_secret_key::LWESecretKey;

// use std::any::type_name;



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

    let bsk = LWEBSK::new(in_key, &rlwe_key, 6, 6);

    return bsk;
}


/*
#### ---- CREATE BOOTSTRAPPING KEY, In key = Out key ---- ####
Input: ( IN KEY )
Output: ( Bootstrap key )
*/
fn create_bsk_one_keys(in_key: &LWESecretKey) -> LWEBSK{

    let rlwe_key = in_key.to_rlwe_secret_key(in_key.dimension).unwrap(); 

    let bsk = LWEBSK::new(in_key, &rlwe_key, 6, 6);

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

// Can something like this work????
/*
pub enum KEY<S,B,K>{
    LWE_key(S),
    LWEBSK_key(B),
    LWEKSK_key(K),
}
pub fn load_key(id: &str, key_type: &str) -> KEY<LWESecretKey, LWEBSK, LWEKSK> {
*/

//     if key_type == "LWE"{
//         return KEY::LWE_key(LWESecretKey::load(&format!("{}{}{}", "keys/", "LWE", id)).unwrap());

//     }else if key_type == "LWEBSK"{
//         return KEY::LWEBSK_key(LWEBSK::load(&format!("{}{}{}", "keys/", "BSK", id)));

//     }else if key_type == "LWEKSK"{
//         return KEY::LWEKSK_key(LWEKSK::load(&format!("{}{}{}", "keys/", "KSK", id)));

//     }else{
//         panic!("Invalid key type!")
//     }
// }

// This will do for now vs. code snippet above
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
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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

// pub fn remove_padding()








/* 
    NUMERIC FUNCTIONS
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
    }
    else{
        if x < a {
            return 1.0;
        }
        else {
            return 0.0;
        }
    }
}

// pub fn reverse_step_function(x: f64, a: f64) -> f64{
//     if x < a {
//         1.0
//     }
//     else {
//         0.0
//     }
// }

