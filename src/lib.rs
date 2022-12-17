use concrete::*;
// use concrete::lwe_secret_key::LWESecretKey;

// use std::any::type_name;

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
pub fn new_bsk_key(in_key: &LWESecretKey, out_key: Option<&LWESecretKey>) -> LWEBSK {

    let bsk_key = match out_key {
        Some(k) => create_bsk_two_key(in_key, k),
        None => create_bsk_one_key(in_key),
    };

    return bsk_key;
}

/*
#### ---- CREATE BOOTSTRAPPING KEY, In key =/= Out key ---- ####
Input: ( IN KEY, OUT KEY )
Output: ( Bootstrap key )
*/
fn create_bsk_two_key(in_key: &LWESecretKey, out_key: &LWESecretKey) -> LWEBSK{
    
    let rlwe_key = out_key.to_rlwe_secret_key(out_key.dimension).unwrap(); 

    let bsk_key = LWEBSK::new(in_key, &rlwe_key, 6, 6);

    return bsk_key;
}


/*
#### ---- CREATE BOOTSTRAPPING KEY, In key = Out key ---- ####
Input: ( IN KEY )
Output: ( Bootstrap key )
*/
fn create_bsk_one_key(in_key: &LWESecretKey) -> LWEBSK{

    let rlwe_key = in_key.to_rlwe_secret_key(in_key.dimension).unwrap(); 

    let bsk_key = LWEBSK::new(in_key, &rlwe_key, 6, 6);

    return bsk_key;
}

/*
#### ---- CREATE NEW KEYSWITCHING KEY ---- ####
Input: ( IN KEY, (Optional) OUT KEY )
Output: ( Keyswitching key )
*/
pub fn new_ksk_key(in_key: &LWESecretKey, out_key: &LWESecretKey) -> LWEKSK {

    let ksk_key = LWEKSK::new(in_key, out_key, 6, 6);

    return ksk_key;
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
pub fn load_sk_key(id: &str) -> LWESecretKey {
    return LWESecretKey::load(&format!("{}{}{}", "keys/", "LWE", id)).unwrap();
}

pub fn load_bsk_key(id: &str) -> LWEBSK {
    return LWEBSK::load(&format!("{}{}{}", "keys/", "BSK", id))
}

pub fn load_ksk_key(id: &str) -> LWEKSK {
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

fn step_function(x: f64, a: f64) -> f64{
    if x > a {
        1.0
    }
    else {
        0.0
    }
}

fn reverse_step_function(x: f64, a: f64) -> f64{
    if x < a {
        1.0
    }
    else {
        0.0
    }
}

