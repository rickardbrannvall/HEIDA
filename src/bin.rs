use concrete::*;
use lib::*;

fn get_dummy_data() -> (small, large) {

    // man:0, smoking:1, diabetic:2, high_blood_pressure:3, alco:4
    let small: Vec<f64> = vec![1., 0., 0., 0., 2.]; 

    // age:0, HDL_chol:1, weight:2, height:3, exercise:4, man: 5
    let large: Vec<f64> = vec![46., 50., 60., 173., 50., 1.];
    
    println!("small features {:?}\n", &small);
    println!("large features {:?}\n", &large);

}

fn VecLWE_to_ListVecLWE() -> (){

}

fn ListVecLWE_to_VecLWE() -> (){
    
}

fn encrypt_cardio_data(small: &Vec<f64>, large: &Vec<f64>, sk: &LWESecretKey) -> (VectorLWE, VectorLWE) {

    // create an encoder
    let enc_small = Encoder::new(0., 15., 6, 1).unwrap();
    let enc_large = Encoder::new(0., 255., 8, 1).unwrap();

    let small_enc = VectorLWE::encode_encrypt(&sk, small, &enc_small).unwrap();
    // println!("small values {:?}", small_enc.decrypt_decode(&sk0).unwrap());
    small_enc.pp();    
    
    let large_enc = VectorLWE::encode_encrypt(&sk, large, &enc_large).unwrap();
    // println!("large values {:?}", large_enc.decrypt_decode(&sk0).unwrap());
    large_enc.pp();    

    return (small_enc, large_enc);
}

pub fn cardio_test() -> () {
    
    let id = "00001_in";
    let sk = load_key(id);

    
    let mut y0 = VectorLWE::zero(1024, 8).unwrap();
    y0.copy_in_nth_nth_inplace(0, &small_enc, 0).unwrap(); // dummy
    y0.copy_in_nth_nth_inplace(1, &small_enc, 1).unwrap(); // smoking
    y0.copy_in_nth_nth_inplace(2, &small_enc, 2).unwrap(); // diabetic
    y0.copy_in_nth_nth_inplace(3, &small_enc, 3).unwrap(); // blood pressure
    y0.copy_in_nth_nth_inplace(4, &small_enc, 0).unwrap(); // dummy
    y0.copy_in_nth_nth_inplace(5, &small_enc, 0).unwrap(); // dummy
    y0.copy_in_nth_nth_inplace(6, &small_enc, 0).unwrap(); // dummy
    y0.copy_in_nth_nth_inplace(7, &small_enc, 0).unwrap(); // dummy
    
    println!("scores {:?}", y0.decrypt_decode(&sk0).unwrap());
    y0.pp();    
    
    
    // *** age check ***
    let mut age0 = large_enc.extract_nth(0).unwrap();
    //let mut mod0 = small_enc.extract_nth(0).unwrap();
    let mut mod0 = large_enc.extract_nth(5).unwrap();
    let diff: Vec<i32> = vec![10];
    mod0.mul_constant_static_encoder_inplace(&diff)?;
    println!("constant mul ok: {:?}", mod0.decrypt_decode(&sk0).unwrap());
    
    age0.add_with_new_min_inplace(&mod0, &vec![0.0])?;
    println!("age_eff {:?}", age0.decrypt_decode(&sk0).unwrap());
    age0.pp(); 

    println!("loading BSK 01... \n");
    let bsk01_path = format!("{}/bsk01_LWE.json",path);
    let bsk01 = LWEBSK::load(&bsk01_path);

    println!("loading KSK 10... \n");
    let ksk10_path = format!("{}/ksk10_LWE.json",path);
    let ksk10 = LWEKSK::load(&ksk10_path);    
    
    let fun = |val:f64| {
        if val>60.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let age1 = age0.bootstrap_nth_with_function(&bsk01, fun, &enc_small, 0)?;
    println!("age_ind {:?}", age1.decrypt_decode(&sk1).unwrap());
    
    age0 = age1.keyswitch(&ksk10).unwrap();
    y0.copy_in_nth_nth_inplace(0, &age0, 0).unwrap();

    
    // *** HDL chol ***
    let mut HDL_chol0 = large_enc.extract_nth(1).unwrap();

    println!("HDL_chol0 {:?}", HDL_chol0.decrypt_decode(&sk0).unwrap());
    HDL_chol0.pp(); 
    
    let fun = |val:f64| {
        if val<40.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let HDL_chol1 = HDL_chol0.bootstrap_nth_with_function(&bsk01, fun, &enc_small, 0)?;
    println!("HDL_chol_ind {:?}", HDL_chol1.decrypt_decode(&sk1).unwrap());
    
    HDL_chol0 = HDL_chol1.keyswitch(&ksk10).unwrap();
    y0.copy_in_nth_nth_inplace(4, &HDL_chol0, 0).unwrap();    

    
    // *** weight check ***
    let mut weight0 = large_enc.extract_nth(2).unwrap();
    let mut height0 = large_enc.extract_nth(3).unwrap();
    weight0.opposite_nth_inplace(0).unwrap();
    height0.add_with_new_min_inplace(&weight0, &vec![0.0])?;

    println!("height_ind {:?}", height0.decrypt_decode(&sk0).unwrap());
    height0.pp(); 
    
    let fun = |val:f64| {
        if val<90.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let height1 = height0.bootstrap_nth_with_function(&bsk01, fun, &enc_small, 0)?;
    println!("height_ind {:?}", height1.decrypt_decode(&sk1).unwrap());
    
    height0 = height1.keyswitch(&ksk10).unwrap();
    y0.copy_in_nth_nth_inplace(5, &height0, 0).unwrap();

    
    // *** physical activity ***
    let mut exercise0 = large_enc.extract_nth(4).unwrap();

    println!("exercise0 {:?}", exercise0.decrypt_decode(&sk0).unwrap());
    exercise0.pp(); 
    
    let fun = |val:f64| {
        if val<30.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let exercise1 = exercise0.bootstrap_nth_with_function(&bsk01, fun, &enc_small, 0)?;
    println!("exercise_ind {:?}", exercise1.decrypt_decode(&sk1).unwrap());
    
    exercise0 = exercise1.keyswitch(&ksk10).unwrap();
    y0.copy_in_nth_nth_inplace(6, &exercise0, 0).unwrap();    
    

    // *** alcohol check ***
    let mut alco0 = small_enc.extract_nth(4).unwrap();
    let mut disc0 = small_enc.extract_nth(0).unwrap();
    let addon: Vec<f64> = vec![1.0];
    alco0.add_constant_static_encoder_inplace(&addon)?;
    println!("constant add ok: {:?}", alco0.decrypt_decode(&sk0).unwrap());
    
    disc0.opposite_nth_inplace(0).unwrap();
    alco0.add_with_new_min_inplace(&disc0, &vec![0.0])?;
    println!("effective alco: {:?}", alco0.decrypt_decode(&sk0).unwrap());
    
    let fun = |val:f64| {
        if val>3.0 {
            1.0
        } else {
            0.0 
        }        
    };
    
    let alco1 = alco0.bootstrap_nth_with_function(&bsk01, fun, &enc_small, 0)?;
    println!("alco_ind {:?}", alco1.decrypt_decode(&sk1).unwrap());
    
    alco0 = alco1.keyswitch(&ksk10).unwrap();
    y0.copy_in_nth_nth_inplace(7, &alco0, 0).unwrap();    
    
    
    // *** display scores
    
    println!("scores {:?}", y0.decrypt_decode(&sk0).unwrap());
    y0.pp();    

    let score0 = y0.sum_with_new_min(0.).unwrap();
    
    println!("score {:?}", score0.decrypt_decode(&sk0).unwrap());
    score0.pp();    
    
    Ok(())
}
