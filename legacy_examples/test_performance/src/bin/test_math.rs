#![allow(non_snake_case)]

fn main() {
    
    // function to calculate slope of CGM curve (assuming constant dt=5 min)
    fn score_GVP(dy: f64) -> f64 {(1.0 + (dy/5.0).powi(2)).sqrt() - 1.0}
    
    println!("score_GVP(0.0) {}", score_GVP(0.0));
    println!("score_GVP(5.0) {}", score_GVP(5.0));
    println!("score_GVP(8.7) {}", score_GVP(8.7));    
    
    
    // function to check if blood glucoce level is in safe range
    fn score_IR(x: f64) -> f64 { if (x >= 70.) && (x <= 180.) {100.0} else {0.0} }
    
    println!("score_IR(65) {}", score_IR(65.));
    println!("score_IR(72) {}", score_IR(72.));
    println!("score_IR(182) {}", score_IR(182.));
    
    
    // function to check for hypoglycemic levels
    fn score_70(x: f64) -> f64 { if x < 70. {100.0} else {0.0} }
    
    println!("score_70(50) {}", score_70(50.));
    println!("score_70(65) {}", score_70(65.));
    println!("score_70(72) {}", score_70(72.));
    
    
    // function to check for severe hypoglycemia
    fn score_54(x: f64) -> f64 { if x < 54. {100.0} else {0.0} }
    
    println!("score_54(50) {}", score_54(50.));
    println!("score_54(65) {}", score_54(65.));
    println!("score_54(72) {}", score_54(72.));

    fn exp(x: f64) -> f64 { x.exp() }
    
    fn scale_54(x: f64) -> f64 { 0.5 + 4.5*(1.0-exp(-0.81093*x/100.)) }
    
    println!("scale_54(5) {}", scale_54(5.0));
    println!("scale_54(50) {}", scale_54(50.0));
    println!("scale_54(95) {}", scale_54(95.0));    
   
    fn scale_70(x: f64) -> f64 { if x <= 7.65 {0.625 + 0.5714*x} else {5.0} }
    
    println!("scale_70(5) {}", scale_70(5.0));
    println!("scale_70(50) {}", scale_70(50.0));
    println!("scale_70(95) {}", scale_70(95.0));    

    fn scale_PTIR(x: f64) -> f64 { 1. + 9./(1.+exp(0.0833*(x-55.05))) }
    
    println!("scale_PTIR(5) {}", scale_PTIR(5.0));
    println!("scale_PTIR(50) {}", scale_PTIR(50.0));
    println!("scale_PTIR(95) {}", scale_PTIR(95.0));    


    fn scale_MG(x: f64) -> f64 { 1. + 9./(1.+exp(0.1139*(x-72.08))) + 9./(1.+exp(-0.09195*(x-157.57))) }
    
    println!("scale_MG(55) {}", scale_MG(55.0));
    println!("scale_MG(120) {}", scale_MG(120.0));
    println!("scale_MG(195) {}", scale_MG(195.0));    

    fn scale_GVP(x: f64) -> f64 { 1. + 9./(1.+exp(-0.049*(x-65.47))) }
    
    println!("scale_GVP(5) {}", scale_GVP(5.0));
    println!("scale_GVP(50) {}", scale_GVP(50.0));
    println!("scale_GVP(95) {}", scale_GVP(95.0));    
    
}