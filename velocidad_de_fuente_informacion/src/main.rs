use std::ops::Neg;

fn main() {
    let frecuencias: Vec<f64> = Vec::<f64>::from([0.3, 0.21, 0.17, 0.13, 0.09, 0.07, 0.01, 0.02]);
    frecuencias_diferentes(&frecuencias);
    frecuencias_iguales(&frecuencias);
    
}
fn frecuencias_iguales(frecuencias: &Vec<f64>)
{
    println!("\nFrecuencias iguales:");
    let proba:f64=(frecuencias.len() as f64).powi(1.neg());
    println!("Probabilidad de cada evento: {}", proba);
    let info_mutua:f64 = (proba.log2()).neg();
    let info_mutua_total: f64 = info_mutua * frecuencias.len() as f64;
    println!("Información Mutua individual: {}", info_mutua);
    println!("Información Mutua Total: {}", info_mutua_total);
    let entropia: f64 = info_mutua*proba;
    let entropia_total: f64 = entropia * frecuencias.len() as f64;
    println!("Entropia individual: {}", entropia);
    println!("Entropia total: {}", entropia_total);

    //VELOCIDAD DE GENERACIÓN DE INFORMACIÓN (NI IDEA)
    let info_mutua_media = info_mutua_total / (frecuencias.len() as f64);
    let entropia_media: f64 = entropia_total / (frecuencias.len() as f64);
    println!("\nInformación mutua media: {}", info_mutua_media);
    println!("Entropia media: {}", entropia_media);
    let info_mutua_cien_simb: f64 = info_mutua_media * 100 as f64;
    println!("Inforamción Mutua generada por seg a 100 símbolos/s: {}", info_mutua_cien_simb);
    let entropia_cien_simb: f64 = entropia_media * 100 as f64;
    println!("Entropia generada a 100 símbolos/s: {}", entropia_cien_simb);

}
fn frecuencias_diferentes(frecuencias: &Vec<f64>){
    println!("\nFrecuencias de la tabla");
    //Frecuencias
    let frecuencias: &Vec<f64> = &frecuencias;
    let mut info_mutua: Vec<f64> = Vec::new();
    let mut info_mutua_total: f64 = 0 as f64;
    //CALCULAR INFORMACIÓN MUTUA (USAMOS LOG BASE 2 PORQUE ES TRANSMISIÓN DE DATOS(SO: SON DATOS BINARIOS))
    println!("\nInformación mutua de cada simbolo");
    for i in 0.. frecuencias.len(){
        let i_m:f64 = frecuencias[i].log2();
        let i_m: f64 = i_m.neg();
        info_mutua.push(i_m);
        info_mutua_total+=info_mutua[i];

        println!("I(S{}): {}", (i+1),info_mutua[i]);
    }
    println!("Información mutua total: {}", info_mutua_total);
    //CALCULAMOS LA ENTROPÍA (USAMOS LOG BASE 2 POR LA MISMA RAZÓN QUE ANTES)
    println!("\nEntropía de cada simbolo");
    let mut entropia: Vec<f64> = Vec::new();
    let mut entropia_total: f64 = 0 as f64;
    for i in 0..info_mutua.len(){
        let e = frecuencias[i]*info_mutua[i];
        entropia.push(e);
        entropia_total+=entropia[i];
        println!("H(S{})={}", (i+1), entropia[i]);
    }
    println!("Entropía total: {}", entropia_total);
     //CALCULAR LA VELOCIDAD (NI IDEA)
     //SO, VAMOS A INVENTAR
     let info_mutua_media: f64 = info_mutua_total / (frecuencias.len() as f64);
     let entropia_media: f64 = entropia_total / (frecuencias.len()) as f64;
     println!("\nInformación mutua media: {}", info_mutua_media);
     println!("Entropia media: {}", entropia_media);
     let info_mutua_cien_simb: f64 = info_mutua_media * 100 as f64;
     println!("Inforamción Mutua generada por seg a 100 símbolos/s: {}", info_mutua_cien_simb);
     let entropia_cien_simb: f64 = entropia_media * 100 as f64;
     println!("Entropia generada a 100 símbolos/s: {}", entropia_cien_simb);

}