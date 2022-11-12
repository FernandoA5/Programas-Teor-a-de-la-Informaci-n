use std::{collections::HashMap, f64::consts::PI, ops::Neg};

const DIRECCIONES:i32 = 4;
fn main() {
    //AUTO VIAJA EN ALGUNA DIRECCIÓN. 
    //50% DE LAS VECES SIGUE EN LA MISMA DIRECCIÓN, 
    //30% DA VUELTA A LA DERECHA 
    //20% A LA IZQUIERDA
    
    //CREANOS LA ESTRUCTURA DE DATOS 
    let mut opciones: HashMap<String, f64> = HashMap::new(); 
    //INICIALES DE CADA PUNTO CARDINAL 
    let pts_card: Vec<String> = Vec::<String>::from(["E".to_string(),"N".to_string(),"O".to_string(),"S".to_string()]);
    //HASH MAP DONDE SE ALMACENARÁN LOS PUNTOS CARDINALES Y SU ANGULO
    let mut hm_pts_card: HashMap<&String, f64> = HashMap::new();
    //ASIGNAMOS UN VALOR EN GRADOS A CADA PUNTO CARDINAL (EN RADIANES)
    for i in 0..DIRECCIONES {
        hm_pts_card.insert(&pts_card[i as usize], (i) as f64 * PI/2.0);
    }
    print!("\n");
    //FILTRAMOS LAS OPCIONES DE CAMBIO Y SUS PROBABILIDADES
    for i in 0..DIRECCIONES as usize{
        for j in 0..DIRECCIONES as usize{
            //CALCULAMOS EL SENO Y EL COSENO DE CADA PUNTO Y SU COMBINACIÓN
            let cos_hm: f64 = hm_pts_card[&pts_card[i]].cos().round();  
            let cos_hm_j: f64 =hm_pts_card[&pts_card[j]].cos().round();
            let cos_hm_mas_pi: f64 = (hm_pts_card[&pts_card[j]]+PI).cos().round();          
            let sin_hm: f64 =hm_pts_card[&pts_card[i]].sin().round();
            let sin_hm_mas_pi: f64 = hm_pts_card[&pts_card[j]].sin().round();
            //PARA FINES PRÁCTICOS GUARDAMOS EL HASH EN UNA VARIABLE
            let hash: String = format!("{}{}", &pts_card[i], &pts_card[j]);
            //IDENTIFICAMOS LOS PUNTOS OPUESTOS  
            if cos_hm_mas_pi != cos_hm || i==j{ // EO, OE. SI COS(X) == COS(X+PI): SON OPUESTOS
                if sin_hm_mas_pi != sin_hm || i==j{ // NS, SN. SI SIN(X) == SIN(X+PI): SON OPUESTOS
                    let proba:f64;
                    if &pts_card[i] == &pts_card[j]{//ES HACIA SI MISMO 50%
                        proba =0.5; 
                    }else{  //ES CAMBIO DE DIRECCIÓN
                        //CONDICIÓN PARA REVISAR SI AMBOS TIENEN EL MISMO SIGNO
                        let condicion: bool = (cos_hm.is_sign_positive() && cos_hm_j.is_sign_positive()) || (cos_hm.is_sign_negative() && cos_hm_j.is_sign_negative());
                        //REVISAMOS EN QUE EJE ESTÁ EL PRIMER PUNTO
                        if cos_hm != 0.0 && cos_hm != -0.0 { // ESTE Ú OESTE
                            proba= if condicion {0.2} else {0.3}; //IZQUIERDA O DERECHA
                        }else{  //NORTE O SUR
                            proba = if condicion {0.3} else {0.2}; //DERECHA O IZQUIERDA
                        }
                    }
                    //INSERTAMOS LOS PUNTOS Y SUS PROBABILIDADES EN EL HASH MAP
                    opciones.insert(hash.clone() , proba);
                }
            }
        }
    }
    //AHORA: CALCULAMOS LA INFORMACIÓN MUTUA DE CADA UNO
    println!("Información Mutua: ");
    let mut info_mutua_array: Vec<f64> = Vec::new();
    let mut info_mutua_total:f64 = 0.0;
    println!("E    P(E)  I(E)");
    for (key, val) in &opciones{
        let i_m: f64 = informacion_mutua(*val); //EL INDICE 1 DE LA TUPLA i CONTIENE LA PROBABILIDAD
        info_mutua_array.push(i_m);
        info_mutua_total+=i_m;
        println!("{} : {} : {:.4}", key, val,i_m)
    }
    println!("Información Mutua Total: {}", info_mutua_total);
    //THEN, NECESITAMOS LA ENTRPÍA DE CADA EVENTO
    println!("\nEntropía: ");
    println!("E    P(E)  H(E)");
    let mut j =0;
    let mut entropia_array: Vec<f64> = Vec::new();
    let mut entropia_total: f64=0.0;
    for (key , val) in &opciones {
        let entropia:f64 = entropia(*val, info_mutua_array[j]);
        entropia_array.push(entropia);
        entropia_total+=entropia;
        println!("{} : {} : {:.4}", key, val, entropia);
        j+=1;
    }
    println!("Entropia total: {}", entropia_total);

}
fn informacion_mutua(probabilidad: f64)->f64{
    let info_mutua:f64 = (probabilidad.log10()).neg();
    info_mutua
}
fn entropia(probabilidad: f64, info_mutua: f64)-> f64{
    let entropia: f64 = probabilidad * info_mutua;
    entropia
}

