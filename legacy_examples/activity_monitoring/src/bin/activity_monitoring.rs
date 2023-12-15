use concrete::*;
use serde::*;

use lib::*;

fn main(){

    let id = "00001";

    let sk = load_sk(id);
    
    let (test_datas, test_labels) = Data::load("test_data");
    let (plain_mus, plain_sigs) = Pred::load("plain_data");

    println!("X: {:?}", &test_datas[0]);
    println!("Y: {:?}\n", &test_labels[0]);
    
    let prec = 7;
    let enc = Encoder::new(-0.07, 1., prec, 6+3+1).unwrap();    

    let input = VectorLWE::encode_encrypt(&sk, &test_datas[0], &enc).unwrap();
    let zero = VectorLWE::encode_encrypt(&sk, &vec![0.0], &enc).unwrap();

    let model_name = "activity_monitoring_model";

    let mut model = Net::load_model(model_name, id);
    model.zero = Some(zero);

    let (mu, sig) = model.forward(input.clone());


    println!("plain mu's: {:?}", &plain_mus[0]);
    println!("enc mu's = {:?}\n", mu.decrypt_decode(&sk).unwrap());

    println!("plain sig's: {:?}", &plain_sigs[0]);
    println!("enc sig's = {:?}", sig.decrypt_decode(&sk).unwrap());
    

}