use concrete::*;

// #### ---- CREATE NEW SECRET KEY ---- ####
// Input: ( Dimmension, Noise level )
// Output: ( Secret key )
pub fn new_key(dimension: usize, noise_level: i32) -> LWESecretKey {

    let lwe_params: LWEParams = LWEParams::new(dimension, noise_level);
    let key = LWESecretKey::new(&lwe_params);

    return key
}

// #### ---- CREATE NEW (PROGRAMABLE) BOOTSTRAPPING KEY ---- ####
// Input: ( IN KEY, (Optional) OUT KEY )
// Output: ( Secret key )
pub fn new_PBS_key(in_key: LWESecretKey, out_key: Option<LWESecretKey>) -> LWEBSK {

    let match: Vec<String> =

    let lwe_params: LWEParams = LWEParams::new(dimension, noise_level);
    let key = LWESecretKey::new(&lwe_params);

    return PBS_key
}

// let lwe_dim = 1024;//512, 1024, 2048];
// let lwe_noise = -40;//-19, -40, -62];

// let rlwe_dim = 1024; //512, 1024, 2048];
// let rlwe_noise = -40; //-19, -40, -62];

// let base_log = 6;
// let lvl = 6;

// //let lwe_params: LWEParams = LWEParams::new(lwe_dim, lwe_noise);
// let rlwe_params: RLWEParams = RLWEParams{polynomial_size: rlwe_dim, dimension: 1, log2_std_dev: rlwe_noise};

// let sk_rlwe = RLWESecretKey::new(&rlwe_params);
// let sk = sk_rlwe.to_lwe_secret_key();
// let bsk = LWEBSK::new(&sk, &sk_rlwe, base_log, lvl);
// //let ksk = LWEKSK::new(&sk, &sk_rlwe, base_log, lvl);