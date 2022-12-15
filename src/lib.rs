use concrete::*;
use concrete::lwe_secret_key::LWESecretKey;

use std::any::type_name;

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
pub fn new_PBS_key(in_key: &LWESecretKey, out_key: Option<&LWESecretKey>) -> LWEBSK {

    let PBS_key = match out_key {
        Some(k) => create_PBS_two_key(in_key, k),
        None => create_PBS_one_key(in_key),
    };

    return PBS_key;
}

/*
#### ---- CREATE BOOTSTRAPPING KEY, In key =/= Out key ---- ####
Input: ( IN KEY, OUT KEY )
Output: ( Bootstrap key )
*/
fn create_PBS_two_key(in_key: &LWESecretKey, out_key: &LWESecretKey) -> LWEBSK{
    
    let rlwe_key = out_key.to_rlwe_secret_key(out_key.dimension).unwrap(); 

    let PBS_key = LWEBSK::new(in_key, &rlwe_key, 6, 6);

    return PBS_key;
}


/*
#### ---- CREATE BOOTSTRAPPING KEY, In key = Out key ---- ####
Input: ( IN KEY )
Output: ( Bootstrap key )
*/
fn create_PBS_one_key(in_key: &LWESecretKey) -> LWEBSK{

    let rlwe_key = in_key.to_rlwe_secret_key(in_key.dimension).unwrap(); 

    let PBS_key = LWEBSK::new(in_key, &rlwe_key, 6, 6);

    return PBS_key;
}

/*
#### ---- CREATE NEW KEYSWITCHING KEY ---- ####
Input: ( IN KEY, (Optional) OUT KEY )
Output: ( Keyswitching key )
*/
pub fn new_KS_key(in_key: &LWESecretKey, out_key: &LWESecretKey) -> LWEKSK {

    let KS_key = LWEKSK::new(in_key, out_key, 6, 6);

    return KS_key;
}

pub trait Save {
    fn type_of(&self) -> &'static str;
}
pub fn save_key(key: &LWESecretKey, id: &str) -> (){
// pub fn save_key<T>(a: &T, id: &str) -> (){
    key.save(&format!("{}{}{}", "keys/", "LWE", id));

}

// format!("{}{}{}","keys/", "LWE", id);

pub fn load_key(id: &str) -> LWESecretKey {
    let key = LWESecretKey::load(&format!("{}{}{}", "keys/", "LWE", id)).unwrap();

    return key;
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


