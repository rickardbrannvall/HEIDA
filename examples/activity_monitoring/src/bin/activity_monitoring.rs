use concrete::*;
use serde::*;

// mod net;
// use net::*;
use lib::*;

fn main(){

    let id = "00001";

    let sk = load_sk(id);

    // let all_keys = (&sk0, &sk0, &bsk, &ksk);
    
    // let n: usize = 12;
    // let N = u32::pow(2, n as u32) as usize;
    
    // println!("Loading data!\n");
    // let data = load_data("data/two_weeks.csv", N);
    
    // let net = Net::new(input_size, *h, output_size);

    // let file = File::open("models/activity_monitoring_model.json").unwrap();
    // let reader = BufReader::new(file);
    // // Read the JSON contents of the file
    // let model: Model = serde_json::from_reader(reader).unwrap();

    let (test_datas, test_labels) = Data::load("test_data");
    
    let (plain_mus, plain_sigs) = Pred::load("plain_data");

    println!("X: {:?}", &test_datas[0]);
    println!("Y: {:?}\n", &test_labels[0]);

    // println!("plain mu: {:?}", &plain_mus[0]);
    // println!("plain sig: {:?}\n", &plain_sigs[0]);


    let mut data = plain_mus[0].clone();

    for _ in 0..26{
        data.push(0.0);
    }
    // println!("data: {:?}", &data);

    let mut sum = 0.;
    for d in &data{
        sum += d;
    }
    println!("sum: {}\n", sum);

    let prec = 6;
    let enc = Encoder::new(0., 110., prec, prec+3+1).unwrap();

    let input = VectorLWE::encode_encrypt(&sk, &data, &enc).unwrap();

    let summed = input.sum();
    // println!("{:?}", summed[0].decrypt_decode(&sk).unwrap());
    println!("{:?}", summed.decrypt_decode(&sk).unwrap());


    let input = VectorLWE::encode_encrypt(&sk, &test_datas[0], &enc).unwrap();
    // println!("{}", 2_i32.pow((test_datas[0].len() as i32).log2().ceil) as usize - test_datas[0].len());
    let zero = VectorLWE::encode_encrypt(&sk, &vec![0.0], &enc).unwrap();
    // let mut input = vec![];
    // for d in test_datas[0].iter(){
    //     input.push(LWE::encode_encrypt(&sk, *d, &enc).unwrap());
    // }


    let model_name = "activity_monitoring_model";


    let mut model = Net::load_model(model_name, id);
    model.zero = Some(zero);

    // let (mu, sig) = model.forward_par(input.clone());
    let (mu, sig) = model.forward(input.clone());


    println!("plain mu: {:?}", &plain_mus[0]);
    println!("mu = {:?}\n", mu.decrypt_decode(&sk).unwrap());

    println!("plain sig: {:?}", &plain_sigs[0]);
    println!("sig = {:?}", sig.decrypt_decode(&sk).unwrap());

    // let mut mus = vec![];
    // for m in mu.iter(){
    //     mus.push(m.decrypt_decode(&sk).unwrap());
    // }

    // let mut sigs = vec![];
    // for s in sig.iter(){
    //     sigs.push(s.decrypt_decode(&sk).unwrap());
    // }

    // println!("encrypyted mu: {:?}", &mus);
    // println!("encrypyted sig: {:?}", &sigs);

}