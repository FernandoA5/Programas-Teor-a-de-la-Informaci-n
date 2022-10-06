use std::ops::Neg;
const DADOS: i32=2;
const CARAS: i32=6;
const MAX: i32=6;
const MIN: i32=1;


//I(E) = log1/P(E) = −log P(E).
//H(E) = - SUMATORA(P(Ei)logP(Ei)  )
fn main() {
    //CALCULAR LA INFORMACIÓN MUTUA Y ENTROPÍA DE LOS EVENTOS
        //a. LANZAR 2 DADOS
        //dos_dados_iguales();
        //b. LANZAR 2 DADOS DE DISTINTO COLOR
        dos_dados_diferente_color();


}

fn _dos_dados_iguales(){
    println!("\nInformación mutua y entropía de lanzar dos dados iguales");
    //OBTENER LA PROBABILIDAD
    //CALCULAR COMBINACIONES (NO REPETICIÓN)  PER > COMB
    let _eventos = CARAS*CARAS;

    let mut sumas_bruto: Vec<i32> = Vec::new();
    let suma_max=MAX+MAX;
    let suma_min = MIN+MIN;
    let mut ocurr_de_cada_suma: Vec<i32> = Vec::new();
    
    for i in 1..CARAS+1 {
        for j in 1..CARAS+1 {
                sumas_bruto.push(i+j);
        }
    }   
    //INICIALIZAMOS EL ARRAY EN CEROS
    for _i in suma_min..suma_max+1 {
        ocurr_de_cada_suma.push(0);
    }
    //CONTAMOS CUANTAS VECES SE REPITIÓ CADA SUMA
    for i in 0..sumas_bruto.len() {
        let indice = sumas_bruto[i]-suma_min;
        ocurr_de_cada_suma[indice as usize] += 1;
    }

    //IMPRIMIMOS
    println!("\nProbabilidades de cada suma");
    for i in 0..ocurr_de_cada_suma.len(){
        println!("P({})={}/{}", (i+suma_min as usize), ocurr_de_cada_suma[i], (MAX.pow(2)));
    }

    //CALCULAMOS LA INFORMACIÓN MUTUA DE CADA UNA
    println!("\nInformación mutua de cada suma");
    let mut info_mutua:Vec<f64> = Vec::new();
    let mut info_mutua_total:f64 =0.0;
    
    for i in 0..ocurr_de_cada_suma.len(){
        let p:f64 = (ocurr_de_cada_suma[i] as f64) / (MAX.pow(2) as f64);
        let i_m: f64 = (p.log10()).neg();
        info_mutua.push(i_m);
        println!("I({})= {}", (i+suma_min as usize), info_mutua[i]);
        info_mutua_total+=info_mutua[i];
    }
    //CALCULAMOS LA INFORMACIÓN MUTUA TOTAL
    println!("Información mutua total: {}", info_mutua_total);

    //CALCULAR LA ENTROPÍA

    println!("\nEntropía de cada evento");
    let mut entropia_arr: Vec<f64> = Vec::new();
    let mut entropia_total: f64=0.0;
    
    for i in 0..info_mutua.len() {
        //CALCULAMOS LA PROBABILIDAD DE QUE OCURRA CADA EVENTO (SUMA)
        let p:f64 = (ocurr_de_cada_suma[i] as f64) / (MAX.pow(2) as f64);
        //CALCULAMOS LA ENTROPÍA: PRODUCTO DE LA I(E) * LA PROBABILIDAD
        let entropia = p * info_mutua[i];
        //GUARDAMOS LA ENTROPÍA EN EL VECTOR
        entropia_arr.push(entropia);
        println!("H({})={}", (i+suma_min as usize), entropia_arr[i]);
        entropia_total+=entropia_arr[i];
    }
    println!("Entropía total: {}", entropia_total);
    // println!("Factorial!: {}", comb_totales);
}
fn dos_dados_diferente_color(){
    println!("\nInformación mutua y entropía de lanzar dos dados de diferente color");
    //OBTENER LA PROBABILIDAD
    let proba: f64 = ((1 as f64) / (CARAS as f64)).powi(DADOS);
    //CALCULAMOS LA INFORMACIÓN MUTUA
    let info_mutua_individual: f64 = (proba.log10()).neg();
    let info_mutua_total:f64 = info_mutua_individual*CARAS.pow(DADOS as u32) as f64;

    println!("Probabilidades de cada evento: {}", proba);
    println!("Información mutua de cada evento: {}", info_mutua_individual);
    println!("Información mutua total: {}", info_mutua_total);

    //CALCULAMOS LA ENTROPÍA
    let entropia_individual:f64 = proba * info_mutua_individual;
    let entropia_total:f64= entropia_individual * CARAS.pow(DADOS as u32) as f64;

    println!("\nEntropia de cada evento: {}", entropia_individual);
    println!("Entropía total: {}", entropia_total);
}
