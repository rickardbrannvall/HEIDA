use pyo3::prelude::*;
use rayon::prelude::*;
use concrete::*;
use serde::*;

#[pyfunction]
fn calculate(n_terms: usize) -> PyResult<f64> {
    let numerator = 4.0;
    let mut denominator = 1.0;
    let mut operation = 1.0;
    let mut pi = 0.0;
    for _ in 0..n_terms {
        pi += operation * (numerator / denominator);
        denominator += 2.0;
        operation *= -1.0;
    }
    Ok(pi)
}

#[pyfunction]
fn add(x: usize, y: usize) -> PyResult<usize> {
    let z = x + y;
    Ok(z)
}

pub trait Save {
    fn save_key(&self, id: &str) -> ();
}
impl Save for LWESecretKey{
    fn save_key(&self, id: &str) -> (){
        self.save(&format!("{}{}{}", "keys/", "LWE", id));
    }
}

fn load_sk(id: &str) -> LWESecretKey {
    return LWESecretKey::load(&format!("{}{}{}", "keys/", "LWE", id)).unwrap();
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
    let encoder = Encoder::new(0., 10., 4, 1).unwrap();

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
    let encoder = Encoder::new(0., 10., 4, 1).unwrap();

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
fn get_list_VecLWE_str(x: Vec<f64>, id: String) -> PyResult<()> {

    let sk = load_sk(&id);
    // let encoder = Encoder::new(0., 10., 4, 1).unwrap();

    // let lwe = VectorLWE::encode_encrypt(&sk, &x, &encoder).unwrap();
    // let lwe_string = serde_json::to_string(&lwe).unwrap();

    // let list_lwe: Vec<VectorLWE> = vec![VectorLWE::zero(1, 1).unwrap(); x.len()];

    let mut list_lwe: Vec<VectorLWE> = vec![VectorLWE::zero(1, 1).unwrap()];
    list_lwe.par_iter_mut().zip(x.par_iter()).for_each(| (lst, val) |{
        *lst = lst.push(VectorLWE::zero(1, 1).unwrap());
        let y = val;
    });

    Ok(())
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
    Ok(())
}