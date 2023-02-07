use pyo3::prelude::*;
// use pyo3::class::basic::PyObjectProtocol;

use rayon::prelude::*;
use concrete::*;
use serde::*;


// SAVING KEYS
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

//LOADING KEYS
pub fn load_sk(id: &str) -> LWESecretKey {
    return LWESecretKey::load(&format!("{}{}{}", "keys/", "LWE", id)).unwrap();
}

pub fn load_bsk(id: &str) -> LWEBSK {
    return LWEBSK::load(&format!("{}{}{}", "keys/", "BSK", id))
}

pub fn load_ksk(id: &str) -> LWEKSK {
    return LWEKSK::load(&format!("{}{}{}", "keys/", "KSK", id))
}



// #[derive(Debug, Serialize, Deserialize)]
// // #[pyclass]
// struct LWE_list{
//     pub lwe_list: Vec<VectorLWE>,
// }

#[pyclass]
#[derive(Debug)]
struct context{
    // #[pyo3(get, set)]
    sk: LWESecretKey,
    sk_o: LWESecretKey,
    bsk: LWEBSK,
    ksk: LWEKSK,
    
    //Used for benchmarkings
    ctxt: Option<Vec<VectorLWE>>,
    ctxt_serialized: Option<String>,
}

//Alternative to importing python functions
pub fn myfunc(func: &str, val: f64) -> f64 {
    if func == "relu"{
        return f64::max(val, 0.0);
    }else{
        return 0.0;
    }
    return 0.0;
}

#[pymethods]
impl context {

    #[staticmethod]
    fn new(dim: usize, noise: i32, bsk_dim: usize, bsk_noise: i32, id: &str) -> Self {

        let lwe_params: LWEParams = LWEParams::new(dim, noise);
        let sk = LWESecretKey::new(&lwe_params);

        let lwe_params_o: LWEParams = LWEParams::new(bsk_dim, bsk_noise);
        let sk_o = LWESecretKey::new(&lwe_params_o);
    
        let rlwe_key = sk_o.to_rlwe_secret_key(sk_o.dimension).unwrap(); 

        let bsk = LWEBSK::new(&sk, &rlwe_key, 6, 9);
        let ksk = LWEKSK::new(&sk_o, &sk, 6, 6);

        sk.save_key(id);
        sk_o.save_key(&(id.to_owned()+"_out"));
        bsk.save_key(id);
        ksk.save_key(id);

        Self {
            sk: sk,
            sk_o: sk_o,
            bsk: bsk,
            ksk: ksk,
            ctxt: None,
            ctxt_serialized: None,
        }
    }

    #[staticmethod]
    fn load(id: &str) -> Self {
        Self {
            sk: load_sk(&id),
            sk_o: load_sk(&(id.to_owned()+"_out")),
            bsk: load_bsk(&id),
            ksk: load_ksk(&id),
            ctxt: None,
            ctxt_serialized: None,
        }
    }

    // fn save(&self, id: &str) -> () {
    //     self.sk.save_key(id);
    //     self.sk_o.save_key(&(id.to_owned()+"_out"));
    //     self.bsk.save_key(id);
    //     self.ksk.save_key(id);
    // }
    
    fn print_dim(&self) -> PyResult<()>{
        println!("encryption dim: {}", self.sk.dimension);
        println!("bootstrapping dim: {}", self.bsk.polynomial_size);
        Ok(())
    }

    fn eval(&self, f: &str, vals: Vec<f64>) -> PyResult<()>{
        println!("{}", f);
        for val in vals.iter(){
            println!("{}({}) = {}", f, &val, myfunc(f, *val));
        }
        Ok(())
    }

    fn pyeval(&self, py: Python, f: PyObject, vals: Vec<f64>) -> PyResult<()>{

        let name = f.getattr(py, "__name__")?;

        let f = |x: f64| {
            let obj: PyObject = f.call1(py, (x,)).unwrap();
            obj.extract::<f64>(py).unwrap()
        };
        
        for val in vals.iter(){
            // println!("{}", f(*val));
            println!("{}({}) = {}", name, &val, f(*val));
        }

        Ok(())
    }

    fn py_ctxt_eval(&self, py: Python, f: PyObject) -> (){//, ctxt_serialized: String) -> (){

        let f = |x: f64| {
            let obj: PyObject = f.call1(py, (x,)).unwrap();
            obj.extract::<f64>(py).unwrap()
        };

        let deserialized: Vec<VectorLWE> = serde_json::from_str(&self.ctxt_serialized.as_ref().unwrap()).unwrap();

        let encoder_o = Encoder::new(0., 1., 4, 1).unwrap();

        for ctxt in deserialized.iter(){
            let ct_tmp = ctxt.bootstrap_nth_with_function(&self.bsk, |x| f(x), &encoder_o, 0).unwrap();
            let ct_after = ct_tmp.keyswitch(&self.ksk).unwrap();
        }
    }

    // fn encrypt(&self, vals: Vec<f64>, min: f64, max: f64, prec: usize, padd: usize) -> PyResult<String> {

    //     let encoder = Encoder::new(min, max, prec, padd).unwrap();

    //     let mut list_lwe: Vec<VectorLWE> = vec![VectorLWE::zero(1, 1).unwrap(); vals.len()];
    //     list_lwe.par_iter_mut().zip(vals.par_iter()).for_each(| (item, val) |{
    //         *item = VectorLWE::encode_encrypt(&self.sk, &[*val], &encoder).unwrap();
    //     });
    //     let list_lwe_string = serde_json::to_string(&list_lwe).unwrap();

    //     Ok(list_lwe_string)

    // }

    // fn decrypt(&self, str_x: String, id: String) -> (){

    //     let x: VectorLWE = serde_json::from_str(&str_x).unwrap();

    // }


    

    // For benchmarking purposes
    fn set_ctxt_test(&mut self, vals: Vec<f64>) -> (){
        
        let encoder = Encoder::new(-10., 10., 7, 1).unwrap();
        let mut list_lwe: Vec<VectorLWE> = vec![VectorLWE::zero(1, 1).unwrap(); vals.len()];
        list_lwe.par_iter_mut().zip(vals.par_iter()).for_each(| (item, val) |{
            *item = VectorLWE::encode_encrypt(&self.sk, &[*val], &encoder).unwrap();
        });

        self.ctxt_serialized = Some(serde_json::to_string(&list_lwe).unwrap());
        self.ctxt = Some(list_lwe);

    }
    // For benchmarking purposes
    fn encrypt_test(&mut self, vals: Vec<f64>) -> (){
        
        let encoder = Encoder::new(-10., 10., 7, 1).unwrap();
        let mut list_lwe: Vec<VectorLWE> = vec![VectorLWE::zero(1, 1).unwrap(); vals.len()];
        list_lwe.par_iter_mut().zip(vals.par_iter()).for_each(| (item, val) |{
            *item = VectorLWE::encode_encrypt(&self.sk, &[*val], &encoder).unwrap();
        });

    }
    // For benchmarking purposes
    fn serialize_test(&self) -> (){
        let serialized = serde_json::to_string(&self.ctxt).unwrap();
    }
    // For benchmarking purposes 
    fn deserialize_test(&self) -> (){
        let deserialized: Vec<VectorLWE> = serde_json::from_str(&self.ctxt_serialized.as_ref().unwrap()).unwrap();
    }
    // For benchmarking purposes 
    fn ctxt_eval_test(&self, py: Python, f: PyObject) -> PyResult<()>{//, ctxt_serialized: String) -> (){

        let name = f.getattr(py, "__name__").unwrap();
        let f = |x: f64| {
            let obj: PyObject = f.call1(py, (x,)).unwrap();
            obj.extract::<f64>(py).unwrap()
        };

        let deserialized: Vec<VectorLWE> = serde_json::from_str(&self.ctxt_serialized.as_ref().unwrap()).unwrap();

        let encoder_o = Encoder::new(-1., 1., 4, 1).unwrap();

        for ctxt in deserialized.iter(){
            let ct_tmp = ctxt.bootstrap_nth_with_function(&self.bsk, |x| f(x), &encoder_o, 0).unwrap();
            let ct_after = ct_tmp.keyswitch(&self.ksk).unwrap();
            println!("plaintext: {}({:?}) = {}", name, &ctxt.decrypt_decode(&self.sk).unwrap(), f(ctxt.decrypt_decode(&self.sk).unwrap().clone()[0]));
            println!("ciphertext: {}({:?}) = {:?}\n", name, &ctxt.decrypt_decode(&self.sk).unwrap(), ct_after.decrypt_decode(&self.sk).unwrap());
        }
        Ok(())
    }
    fn ctxt_eval_bench(&self, py: Python, f: PyObject) -> PyResult<()>{//, ctxt_serialized: String) -> (){

        let f = |x: f64| {
            let obj: PyObject = f.call1(py, (x,)).unwrap();
            obj.extract::<f64>(py).unwrap()
        };

        let deserialized: Vec<VectorLWE> = serde_json::from_str(&self.ctxt_serialized.as_ref().unwrap()).unwrap();

        let encoder_o = Encoder::new(-1., 1., 4, 1).unwrap();

        for ctxt in deserialized.iter(){
            let ct_tmp = ctxt.bootstrap_nth_with_function(&self.bsk, |x| f(x), &encoder_o, 0).unwrap();
            let ct_after = ct_tmp.keyswitch(&self.ksk).unwrap();
        }
        Ok(())
    }

}



#[pymodule]
#[pyo3(name = "libheida")]
fn libheida(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<context>()?;
    Ok(())
}