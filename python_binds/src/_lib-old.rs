use pyo3::prelude::*;
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

#[pyfunction]
fn create_sk(dim: usize, noise: i32, id: String) -> PyResult<()> {
    let lwe_params: LWEParams = LWEParams::new(dim, noise);
    let sk = LWESecretKey::new(&lwe_params);

    sk.save_key(&id);    

    Ok(())
}

#[pyfunction]
fn get_LWE_str(x: f64, id: String) -> PyResult<String> {

    let sk = load_sk(&id);
    let encoder = Encoder::new(-5., 5., 4, 2).unwrap();

    let lwe = LWE::encode_encrypt(&sk, x, &encoder).unwrap();
    let lwe_string = serde_json::to_string(&lwe).unwrap();

    Ok(lwe_string)
}

#[pyfunction]
fn add_LWE_str(str_x: String, str_y: String, id: String) -> PyResult<String> {

    let sk = load_sk(&id);

    let x: LWE = serde_json::from_str(&str_x).unwrap();
    let y: LWE = serde_json::from_str(&str_y).unwrap();

    // let z = (x.add_with_padding(&y).unwrap()).decrypt_decode(&sk).unwrap();
    let z = x.add_with_padding(&y).unwrap();
    let str_z = serde_json::to_string(&z).unwrap();

    Ok(str_z)
}

#[pyfunction]
fn decrypt_LWE_str(str_x: String, id: String) -> PyResult<f64> {
    
    let sk = load_sk(&id);

    let x: LWE = serde_json::from_str(&str_x).unwrap();
    
    let res = x.decrypt_decode(&sk).unwrap();

    Ok(res)
}

#[pyfunction]
fn get_VecLWE_str(x: Vec<f64>, id: String) -> PyResult<String> {

    let sk = load_sk(&id);
    let encoder = Encoder::new(-5., 5., 4, 2).unwrap();

    let lwe = VectorLWE::encode_encrypt(&sk, &x, &encoder).unwrap();
    let lwe_string = serde_json::to_string(&lwe).unwrap();

    Ok(lwe_string)
}

#[pyfunction]
fn add_VecLWE_str(str_x: String, str_y: String, id: String) -> PyResult<String> {

    let sk = load_sk(&id);

    let x: VectorLWE = serde_json::from_str(&str_x).unwrap();
    let y: VectorLWE = serde_json::from_str(&str_y).unwrap();

    // let z = (x.add_with_padding(&y).unwrap()).decrypt_decode(&sk).unwrap();
    let z = x.add_with_padding(&y).unwrap();
    let str_z = serde_json::to_string(&z).unwrap();

    Ok(str_z)
}

#[pyfunction]
fn decrypt_VecLWE_str(str_x: String, id: String) -> PyResult<Vec<f64>> {
    
    let sk = load_sk(&id);

    let x: VectorLWE = serde_json::from_str(&str_x).unwrap();
    
    let res = x.decrypt_decode(&sk).unwrap();

    Ok(res)
}

#[pyfunction]
fn get_list_VecLWE_str(x: Vec<f64>, id: String) -> PyResult<(String)> {

    let sk = load_sk(&id);
    let encoder = Encoder::new(-5., 5., 4, 2).unwrap();

    let mut list_lwe: Vec<VectorLWE> = vec![VectorLWE::zero(1, 1).unwrap(); x.len()];

    list_lwe.par_iter_mut().zip(x.par_iter()).for_each(| (item, val) |{
        *item = VectorLWE::encode_encrypt(&sk, &[*val], &encoder).unwrap();
    });

    let list_lwe_string = serde_json::to_string(&list_lwe).unwrap();

    Ok(list_lwe_string)
}



#[derive(Serialize, Deserialize)]
#[pyclass]
struct LWE_list{
    pub lwe_list: Vec<VectorLWE>,
}

#[pyclass]
struct context{
    // #[pyo3(get, set)]
    sk: LWESecretKey,
    sk_o: LWESecretKey,
    bsk: LWEBSK,
    ksk: LWEKSK,
}

#[pymethods]
impl context {
    #[new]
    fn create_keys(dim: usize, noise: i32, bsk_dim: usize, bsk_noise: i32, id: &str) -> Self {

        let lwe_params: LWEParams = LWEParams::new(dim, noise);
        let sk = LWESecretKey::new(&lwe_params);

        let lwe_params_o: LWEParams = LWEParams::new(bsk_dim, bsk_noise);
        let sk_o = LWESecretKey::new(&lwe_params_o);
    
        let rlwe_key = sk_o.to_rlwe_secret_key(sk_o.dimension).unwrap(); 

        let bsk = LWEBSK::new(in_key, &rlwe_key, 6, 9);

        let ksk = LWEKSK::new(in_key, sk_o, 6, 6);

        context {
            sk: sk,
            sk_o: sk_o,
            bsk: bsk,
            ksk: ksk,
        }
    }
    #[new]
    fn load_keys(id: &str) -> Self {

        context {
            sk: load_sk(&id),
            sk_o: load_sk(&(id.to_owned()+"_out")),
            bsk: load_bsk(&id),
            ksk: load_ksk(&id),
        }
    }
}



#[pymodule]
fn libheida(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(create_sk, m)?)?;

    m.add_function(wrap_pyfunction!(get_LWE_str, m)?)?;
    m.add_function(wrap_pyfunction!(add_LWE_str, m)?)?;
    m.add_function(wrap_pyfunction!(decrypt_LWE_str, m)?)?;

    m.add_function(wrap_pyfunction!(get_VecLWE_str, m)?)?;
    m.add_function(wrap_pyfunction!(add_VecLWE_str, m)?)?;
    m.add_function(wrap_pyfunction!(decrypt_VecLWE_str, m)?)?;

    m.add_function(wrap_pyfunction!(get_list_VecLWE_str, m)?)?;

    m.add_class::<py_keys>()?;
    Ok(())
}