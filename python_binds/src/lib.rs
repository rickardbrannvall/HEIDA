use pyo3::prelude::*;
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
fn create_sk(id: String) -> PyResult<()> {
    let lwe_params: LWEParams = LWEParams::new(1024, -40);
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
fn add_LWE_str(str_x: String, str_y: String, id: String) -> PyResult<f64> {

    let sk = load_sk(&id);

    let x: LWE = serde_json::from_str(&str_x).unwrap();
    let y: LWE = serde_json::from_str(&str_y).unwrap();

    let z = (x.add_with_padding(&y).unwrap()).decrypt_decode(&sk).unwrap();

    Ok(z)
}

#[pymodule]
fn encrypt_val(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(create_sk, m)?)?;
    m.add_function(wrap_pyfunction!(get_LWE_str, m)?)?;
    m.add_function(wrap_pyfunction!(add_LWE_str, m)?)?;
    Ok(())
}