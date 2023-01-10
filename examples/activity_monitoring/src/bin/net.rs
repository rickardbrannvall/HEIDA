use serde::*;

#[derive(Debug, Serialize, Deserialize)]
struct Layer{
    weights: Vec<Vec<f64>>,
    bias: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Model{
    Layer_0: Layer,
    Layer_1: Layer,
    Layer_2: Layer,
    Layer_mu: Layer,
    Layer_sig: Layer,
}

pub struct Net{
    bsk: LWEBSK,
    ksk: Option<LWEKSK>,
    Layer_0: Vec<Vec<f64>>,
    Layer_0_bias: Vec<f64>,
    Layer_1: Vec<Vec<f64>>,
    Layer_1_bias: Vec<f64>,
    Layer_2: Vec<Vec<f64>>,
    Layer_2_bias: Vec<f64>,
    Layer_mu: Vec<Vec<f64>>,
    Layer_sig: Vec<Vec<f64>>,
}
impl Net{

    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self{
        let c = 0.0;
        Net{
            Hidden_size: hidden_size,
            Output_size: output_size,
            Layer_1: vec![vec![c; input_size]; hidden_size],
            Layer_1: vec![vec![c; input_size]; hidden_size],
            Layer_2: vec![vec![c; hidden_size]; hidden_size],
            Layer_mu: vec![vec![c; hidden_size]; output_size],
            Layer_sig: vec![vec![c; hidden_size]; output_size],
        }
    }

    pub fn load_model() -> Self{

    }

    pub fn forward(&self, input: VectorLWE, bsk: &LWEBSK) -> (VectorLWE, VectorLWE){

        let bit_precision = input.encoders[0].nb_bit_precision;
        let enc_out = Encoder::new(0., 5., bit_precision, bit_precision+3).unwrap();

        let mut output_1 = VectorLWE::zero(input.dimension, self.Hidden_size).unwrap();
        let mut output_2 = VectorLWE::zero(input.dimension, self.Hidden_size).unwrap();
        let mut output_mu = VectorLWE::zero(input.dimension, self.Output_size).unwrap();
        let mut output_sig = VectorLWE::zero(input.dimension, self.Output_size).unwrap();

        //input.pp();
        for (i, weights) in self.Layer_1.iter().enumerate(){
            let mut ct_tmp = input.mul_constant_with_padding(weights, 1., bit_precision).unwrap();
            //ct_tmp.pp();
            ct_tmp = sum_N_VectorLWE(&ct_tmp);
            //add bias

            /*let ct_tmp2 = ct_tmp.bootstrap_nth_with_function(&bsk, |x| relu(x), &enc_out, 0).unwrap();
            println!("Here");
            ct_tmp2.pp();
            output_1.copy_in_nth_nth_inplace(i, &ct_tmp, 0).unwrap();*/
            output_1.copy_in_nth_nth_inplace(i, &(ct_tmp.bootstrap_nth_with_function(&bsk, |x| relu(x), &enc_out, 0).unwrap()), 0).unwrap();
            
        }

        //output_1.pp();
        for (i, weights) in self.Layer_2.iter().enumerate(){
            let mut ct_tmp = output_1.mul_constant_with_padding(weights, 1., bit_precision).unwrap();
            //ct_tmp.pp();
            ct_tmp = sum_N_VectorLWE(&ct_tmp);
            //ct_tmp.pp();
            //add bias
            output_2.copy_in_nth_nth_inplace(i, &(ct_tmp.bootstrap_nth_with_function(&bsk, |x| relu(x), &enc_out, 0).unwrap()), 0).unwrap();
        }

        for (i, weights) in self.Layer_mu.iter().enumerate(){
            let mut ct_tmp = output_2.mul_constant_with_padding(weights, 1., bit_precision).unwrap();
            ct_tmp = sum_N_VectorLWE(&ct_tmp);
            //ct_tmp.pp();
            //add bias
            output_mu.copy_in_nth_nth_inplace(i, &ct_tmp, 0).unwrap();
        }

        for (i, weights) in self.Layer_sig.iter().enumerate(){
            let mut ct_tmp = output_2.mul_constant_with_padding(weights, 1., bit_precision).unwrap();
            ct_tmp = sum_N_VectorLWE(&ct_tmp);
            //ct_tmp.pp();
            //add bias
            output_sig.copy_in_nth_nth_inplace(i, &(ct_tmp.bootstrap_nth_with_function(&bsk, |x| elu_plus_one(x), &enc_out, 0).unwrap()), 0).unwrap();
        }
        
        return (output_mu, output_sig);
    }

    /*fn quantize_vectors(&self, )
    */
    fn calculate_layer(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>, func: fn(f64) -> f64, bsk: &LWEBSK, enc: &Encoder, prec: usize) -> Vec<LWE> {
        
        let mut output = vec![input[0].clone(); layer_weights.len()];

        output.par_iter_mut().zip(layer_weights.par_iter()).for_each(| (out, weights) |{
            
            let mut layer_tmp = input.clone();
            weights.par_iter().zip(input.par_iter().zip(layer_tmp.par_iter_mut())).for_each(| (w, (ct_i, ct_o)) |{
                *ct_o = ct_i.mul_constant_with_padding(*w, 1., prec).unwrap();
            });

            let ct_tmp = &sum_N_LWE(&layer_tmp)[0];
            *out = ct_tmp.bootstrap_with_function(&bsk, |x| func(x), &enc).unwrap();
        });

        return output;
    }

    fn calculate_layer_nofunc(&self, input: Vec<LWE>, layer_weights: &Vec<Vec<f64>>, prec: usize) -> Vec<LWE> {
        
        let mut output = vec![input[0].clone(); layer_weights.len()];

        output.par_iter_mut().zip(layer_weights.par_iter()).for_each(| (out, weights) |{
            
            let mut layer_tmp = input.clone();
            weights.par_iter().zip(input.par_iter().zip(layer_tmp.par_iter_mut())).for_each(| (w, (ct_i, ct_o)) |{
                *ct_o = ct_i.mul_constant_with_padding(*w, 1., prec).unwrap();
            });

            //let ct_tmp = sum_N_LWE(&layer_tmp);
            *out = sum_N_LWE(&layer_tmp)[0].clone();//.bootstrap_with_function(&bsk, |x| func(x), &enc).unwrap();
        });

        return output;
    }

    pub fn forward_par(&self, mut input: Vec<LWE>, bsk: &LWEBSK) -> (Vec<LWE>, Vec<LWE>){

        let bit_precision = input[0].encoder.nb_bit_precision;
        let enc_out = Encoder::new(0., 5., bit_precision, bit_precision+3).unwrap();

        let ct_layer_1_o = self.calculate_layer(input, &self.Layer_1, relu, bsk, &enc_out, bit_precision.clone());

        let ct_layer_2_o = self.calculate_layer(ct_layer_1_o, &self.Layer_2, relu, bsk, &enc_out, bit_precision.clone());

        let output_mu = self.calculate_layer_nofunc(ct_layer_2_o.clone(), &self.Layer_mu, bit_precision.clone());

        let output_sig = self.calculate_layer(ct_layer_2_o.clone(), &self.Layer_sig, elu_plus_one, bsk, &enc_out, bit_precision.clone());

        return (output_mu, output_sig);
    }
}
