use concrete::*;

// #### ---- CREATE NEW SECRET KEY ---- ####
pub fn new_key(in_dim: uint, in_noise: uint) -> LWESecretKey {

    let lwe_params: LWEParams = LWEParams::new(in_dim, in_noise);
    let key = LWESecretKey::new(&lwe_params);

    return key
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