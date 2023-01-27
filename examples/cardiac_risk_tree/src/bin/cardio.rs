use concrete::*;
use lib::*;
use std::time::{Duration, Instant};
use rayon::prelude::*;

fn get_dummy_data() -> (Vec<f64>, Vec<f64>) {

    // man:0, smoking:1, diabetic:2, high_blood_pressure:3, alco:4
    let small: Vec<f64> = vec![1., 0., 0., 0., 2.]; 

    // age:0, HDL_chol:1, weight:2, height:3, exercise:4, man: 5
    let large: Vec<f64> = vec![46., 50., 60., 173., 50., 1.];
    
    println!("small features {:?}\n", &small);
    println!("large features {:?}\n", &large);

    return (small, large)

}

fn encrypt_cardio_data(small: &Vec<f64>, large: &Vec<f64>, sk: &LWESecretKey) -> (VectorLWE, VectorLWE, Encoder, Encoder) {

    // create an encoder
    let enc_small = Encoder::new(0., 15., 4, 1).unwrap();
    let enc_large = Encoder::new(0., 255., 4, 1).unwrap();

    let small_enc = VectorLWE::encode_encrypt(&sk, small, &enc_small).unwrap();
    
    let large_enc = VectorLWE::encode_encrypt(&sk, large, &enc_large).unwrap();

    return (small_enc, large_enc, enc_small, enc_large);
}

fn sequential_cardio(small_enc: &VectorLWE, large_enc: &VectorLWE, enc_small: &Encoder, sk: &LWESecretKey, bsk: &LWEBSK, ksk: &LWEKSK) -> (){

    let mut y0 = VectorLWE::zero(1024, 8).unwrap();
    y0.copy_in_nth_nth_inplace(0, &small_enc, 0).unwrap(); // dummy
    y0.copy_in_nth_nth_inplace(1, &small_enc, 1).unwrap(); // smoking
    y0.copy_in_nth_nth_inplace(2, &small_enc, 2).unwrap(); // diabetic
    y0.copy_in_nth_nth_inplace(3, &small_enc, 3).unwrap(); // blood pressure
    y0.copy_in_nth_nth_inplace(4, &small_enc, 0).unwrap(); // dummy
    y0.copy_in_nth_nth_inplace(5, &small_enc, 0).unwrap(); // dummy
    y0.copy_in_nth_nth_inplace(6, &small_enc, 0).unwrap(); // dummy
    y0.copy_in_nth_nth_inplace(7, &small_enc, 0).unwrap(); // dummy
    
    // println!("scores {:?}", y0.decrypt_decode(&sk).unwrap());
        
    
    // *** age check ***
    let mut age0 = large_enc.extract_nth(0).unwrap();
    let mut mod0 = large_enc.extract_nth(5).unwrap();
    let diff: Vec<i32> = vec![10];
    mod0.mul_constant_static_encoder_inplace(&diff).unwrap();
    // println!("constant mul ok: {:?}", mod0.decrypt_decode(&sk).unwrap());
    
    age0.add_with_new_min_inplace(&mod0, &vec![0.0]).unwrap();
    // println!("age_eff {:?}", age0.decrypt_decode(&sk).unwrap());
    
    let fun = |val:f64| {
        if val>60.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let age1 = age0.bootstrap_nth_with_function(&bsk, fun, &enc_small, 0).unwrap();
    // println!("age_ind {:?}", age1.decrypt_decode(&sk).unwrap());
    
    age0 = age1.keyswitch(&ksk).unwrap();
    y0.copy_in_nth_nth_inplace(0, &age0, 0).unwrap();

    
    // *** HDL chol ***
    let mut HDL_chol0 = large_enc.extract_nth(1).unwrap();

    // println!("HDL_chol0 {:?}", HDL_chol0.decrypt_decode(&sk).unwrap());
    
    let fun = |val:f64| {
        if val<40.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let HDL_chol1 = HDL_chol0.bootstrap_nth_with_function(&bsk, fun, &enc_small, 0).unwrap();
    // println!("HDL_chol_ind {:?}", HDL_chol1.decrypt_decode(&sk).unwrap());
    
    HDL_chol0 = HDL_chol1.keyswitch(&ksk).unwrap();
    y0.copy_in_nth_nth_inplace(4, &HDL_chol0, 0).unwrap();    

    
    // *** weight check ***
    let mut weight0 = large_enc.extract_nth(2).unwrap();
    let mut height0 = large_enc.extract_nth(3).unwrap();
    weight0.opposite_nth_inplace(0).unwrap();
    height0.add_with_new_min_inplace(&weight0, &vec![0.0]).unwrap();

    // println!("height_ind {:?}", height0.decrypt_decode(&sk).unwrap());
    
    let fun = |val:f64| {
        if val<90.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let height1 = height0.bootstrap_nth_with_function(&bsk, fun, &enc_small, 0).unwrap();
    // println!("height_ind {:?}", height1.decrypt_decode(&sk).unwrap());
    
    height0 = height1.keyswitch(&ksk).unwrap();
    y0.copy_in_nth_nth_inplace(5, &height0, 0).unwrap();

    
    // *** physical activity ***
    let mut exercise0 = large_enc.extract_nth(4).unwrap();

    // println!("exercise0 {:?}", exercise0.decrypt_decode(&sk).unwrap());
    
    let fun = |val:f64| {
        if val<30.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let exercise1 = exercise0.bootstrap_nth_with_function(&bsk, fun, &enc_small, 0).unwrap();
    // println!("exercise_ind {:?}", exercise1.decrypt_decode(&sk).unwrap());
    
    exercise0 = exercise1.keyswitch(&ksk).unwrap();
    y0.copy_in_nth_nth_inplace(6, &exercise0, 0).unwrap();    
    

    // *** alcohol check ***
    let mut alco0 = small_enc.extract_nth(4).unwrap();
    let mut disc0 = small_enc.extract_nth(0).unwrap();
    let addon: Vec<f64> = vec![1.0];
    alco0.add_constant_static_encoder_inplace(&addon).unwrap();
    // println!("constant add ok: {:?}", alco0.decrypt_decode(&sk).unwrap());
    
    disc0.opposite_nth_inplace(0).unwrap();
    alco0.add_with_new_min_inplace(&disc0, &vec![0.0]).unwrap();
    // println!("effective alco: {:?}", alco0.decrypt_decode(&sk).unwrap());
    
    let fun = |val:f64| {
        if val>3.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let alco1 = alco0.bootstrap_nth_with_function(&bsk, fun, &enc_small, 0).unwrap();
    // println!("alco_ind {:?}", alco1.decrypt_decode(&sk).unwrap());
    
    alco0 = alco1.keyswitch(&ksk).unwrap();
    y0.copy_in_nth_nth_inplace(7, &alco0, 0).unwrap();    
    
    
    // *** display scores
    // println!("scores {:?} ", y0.decrypt_decode(&sk).unwrap());

    let score0 = y0.sum_with_new_min(0.).unwrap();
    
    println!("cardiac score: {:?} (sequential version)", score0.decrypt_decode(&sk).unwrap());
}

fn parallel_cardio(small_enc_new: &VectorLWE, large_enc_new: &VectorLWE, enc_small: &Encoder, sk: &LWESecretKey, bsk: &LWEBSK) -> (){

    // ## ---- Pre-proceessing before Bootstrapping ---- ## 

    // List that will be bootstrapped
    let mut boot_list = vec![];
    let mut func_list = vec![];

    // get VectorLWE in List(VectorLWE) form, makes it possible to do concurrency
    let (small_enc_list, large_enc_list) = (VecLWE_to_ListVecLWE(&small_enc_new), VecLWE_to_ListVecLWE(&large_enc_new));

    // ## -- age -- ##
    let mut age = large_enc_list[0].clone();
    let mut mod0 = large_enc_list[5].clone();
    mod0.mul_constant_static_encoder_inplace(&vec![10]).unwrap();
    age.add_with_new_min_inplace(&mod0, &vec![0.0]).unwrap();
    
    boot_list.push(age);
    func_list.push((step_function, 60.0, 1));

    // ## -- HDL chol -- ##
    boot_list.push(large_enc_list[1].clone());
    func_list.push((step_function, 40.0, -1));

    // ## -- weight check -- ##
    let mut weight = large_enc_list[2].clone();
    let mut height = large_enc_list[3].clone();
    weight.opposite_nth_inplace(0).unwrap();
    height.add_with_new_min_inplace(&weight, &vec![0.0]).unwrap();

    boot_list.push(height);
    func_list.push((step_function, 90.0, -1));

    // ## -- physical activity -- ## 
    boot_list.push(large_enc_list[4].clone());
    func_list.push((step_function, 30.0, -1));

    // ## -- alcohol check -- ## 
    let mut alco = small_enc_list[4].clone();
    let mut disc = small_enc_list[0].clone();
    alco.add_constant_static_encoder_inplace(&vec![1.0]).unwrap();
    disc.opposite_nth_inplace(0).unwrap();
    alco.add_with_new_min_inplace(&disc, &vec![0.0]).unwrap();

    boot_list.push(alco);
    func_list.push((step_function, 3.0, 1));

    // ## -- bootstrapping -- ##
    boot_list.par_iter_mut().zip(func_list.par_iter()).for_each(| (var, (func, val, opt)) |{
        *var = var.bootstrap_nth_with_function(&bsk, |x| func(x, *val, *opt), &enc_small, 0).unwrap();
    });

    boot_list.push(small_enc_list[1].clone());
    boot_list.push(small_enc_list[2].clone());
    boot_list.push(small_enc_list[3].clone());

    // ## -- scores -- ##

    let scores = ListVecLWE_to_VecLWE(boot_list);
    // println!("scores {:?}", scores.decrypt_decode(&sk).unwrap());

    let score = scores.sum_with_new_min(0.).unwrap();
    println!("cardiac score: {:?} (parallel version)", score.decrypt_decode(&sk).unwrap());

}

fn main() {
    
    let id = "00001";

    println!("Loading Secret key!");
    let sk = load_sk(id);

    let (small, large) = get_dummy_data();
    let (small_enc, large_enc, enc_small, enc_large) = encrypt_cardio_data(&small, &large, &sk);

    let (small_enc_new, large_enc_new) = (small_enc.clone(), large_enc.clone());  
    
    println!("Loading Bootstrapping key!\n");
    let bsk = load_bsk(id);

    println!("Loading Keyswitching key!\n");
    let ksk = load_ksk(id);

    /* 
        Sequential version
    */
    let now = Instant::now();

    sequential_cardio_cardio(&small_enc, &large_enc, &enc_small, &sk, &bsk, &ksk);

    println!("elapsed in: {} seconds\n", (now.elapsed().as_millis() as f32)/1000.0);


    /* 
        Parallel version ~4x faster
    */
    let now = Instant::now();

    parallel_cardio(&small_enc_new, &large_enc_new, &enc_small, &sk, &bsk);

    println!("elapsed in: {} seconds\n", (now.elapsed().as_millis() as f32)/1000.0);


}
