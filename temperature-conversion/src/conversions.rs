
pub fn celesius_to_fahenheit(measure: f32) -> f32{
    measure * 1.8 + 32.0
}

pub fn fahenheit_to_celesius(measure: f32) -> f32 {
    (measure - 32.0) / 1.8
}


pub fn target_unit_convert(measure: f32, unit: &str) -> f32{
    if "F".eq(unit){
        celesius_to_fahenheit(measure)
    }else{
        fahenheit_to_celesius(measure)
    }
}