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


    // let mut test_d = test_datas[0].clone();
    let mut test_d = vec![0.5; 19];

    for i in 0..24-19{
        test_d.push(-3.);
    }
    println!("test_d: {:?}", &test_d);

    let mut sum = 0.;
    for d in &test_d{
        sum += d;
    }
    println!("sum: {}\n", sum);

    
    let prec = 4;
    // let enc = Encoder::new(-4.5, 9., prec, prec+2+1).unwrap();
    let enc = Encoder::new(-3., 9., prec, prec+2+1).unwrap();

    let test = VectorLWE::encode_encrypt(&sk, &test_d, &enc).unwrap();

    let summed = test.sum(&sk);
    // println!("{:?}", summed[0].decrypt_decode(&sk).unwrap());
    println!("{:?}", summed.decrypt_decode(&sk).unwrap());

    
    let x = vec![10.];
    let y = vec![-80.];
    let r = vec![-70.];
    let s = vec![-130.];
    let t = vec![-160.];

    let X = VectorLWE::encode_encrypt(&sk, &x, &Encoder::new(0., 40., 5, 0).unwrap()).unwrap();
    let Y = VectorLWE::encode_encrypt(&sk, &y, &Encoder::new(-120., -80., 5, 0).unwrap()).unwrap();
    let R = VectorLWE::encode_encrypt(&sk, &r, &Encoder::new(-110., -70., 5, 0).unwrap()).unwrap();
    let S = VectorLWE::encode_encrypt(&sk, &s, &Encoder::new(-130., -90., 5, 0).unwrap()).unwrap();
    let T = VectorLWE::encode_encrypt(&sk, &t, &Encoder::new(-160., -120., 5, 0).unwrap()).unwrap();

    let mut z  = vec![x[0], y[0], r[0], s[0], t[0]];
    println!("plain sum: {}", z.into_iter().sum::<f64>());
    // X.add_with_new_min_inplace(&Y, &vec![-110.]).unwrap();

    let Z = ListVecLWE_to_VecLWE(vec![X, Y, R, S, T]);
    // println!("{:?}", Z.clone().encoders);

    let minimum = Z.encoders.clone().into_iter().min_by(|a, b| a.o.partial_cmp(&b.o).unwrap()).unwrap().o;
    println!("min: {}", minimum);

    let delta = Z.clone().encoders[0].delta;
    println!("delta: {}", delta);

    let prec: i32 = (Z.clone().encoders.into_iter().map(|s| (s.nb_bit_precision as f64)/(Z.nb_ciphertexts as f64)).sum::<f64>() as f64).round() as i32;
    println!("prec: {}", prec);

    let range = delta * (f64::powi(2., prec) - 1.)/ (f64::powi(2., prec));
    println!("range: {}", range);


    let center: f64  = Z.clone().encoders.into_iter().map(|s| (2.*s.o + range)/2.).sum::<f64>();
    println!("center: {}", center);

    let potential_min = center - range/2.;
    println!("pot_min: {}", potential_min);

    let new_min = f64::min(minimum, potential_min);
    println!("new_min: {}", new_min);
    
    println!("enc sum: {:?}", Z.sum_with_new_min(new_min).unwrap().decrypt_decode(&sk).unwrap());
    
    /*

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
    */

}