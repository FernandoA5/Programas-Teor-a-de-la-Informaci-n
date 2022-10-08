use std::{collections::HashMap, f64::consts::PI};

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
    //ASIGNAMOS UN VALOR EN GRADOS A CADA PUNTO CARDINAL
    for i in 0..DIRECCIONES {
        hm_pts_card.insert(&pts_card[i as usize], (i) as f64 * PI/2.0);
        println!("{}:{:.4}", &pts_card[i as usize], hm_pts_card[&pts_card[i as usize]]);
    }
    print!("\n");
    //FILTRAMOS LAS OPCIONES DE CAMBIO
    for i in 0..DIRECCIONES as usize{
        for j in 0..DIRECCIONES as usize{
            //CALCULAMOS EL SENO Y EL COSENO DE CADA PUNTO Y SU COMBINACIÓN
            let cos_hm: f64 = hm_pts_card[&pts_card[i]].cos().round();  
            let cos_hm_j: f64 =hm_pts_card[&pts_card[j]].cos().round();
            let cos_hm_mas_pi: f64 = (hm_pts_card[&pts_card[j]]+PI).cos().round();          
            let sin_hm: f64 =hm_pts_card[&pts_card[i]].sin().round();
            let sin_hm_j: f64 =hm_pts_card[&pts_card[j]].sin().round();
            let sin_hm_mas_pi: f64 = hm_pts_card[&pts_card[j]].sin().round();
            let hash: String = format!("{}{}", &pts_card[i], &pts_card[j]);
            //IDENTIFICAMOS LOS PUNTOS OPUESTOS
            if cos_hm_mas_pi != cos_hm || i==j{
                if sin_hm_mas_pi != sin_hm || i==j{
                    let mut proba:f64=0.0;
                    if &pts_card[i] == &pts_card[j]{
                        //ES HACIA SI MISMO 50%
                        proba =0.5; 
                    }else{  //ES CAMBIO DE DIRECCIÓN
                        //REVISAMOS QUE AMBOS TENGAN EL MISMO SIGNO
                        let condicion: bool = (cos_hm.is_sign_positive() && cos_hm_j.is_sign_positive()) || (cos_hm.is_sign_negative() && cos_hm_j.is_sign_negative());
                        //VERIFICAMOS QUE VAYA HACIA EL SUR O HACIA EL NORTE
                        if cos_hm != 0.0 && cos_hm != -0.0 {
                            proba= if condicion {0.2} else {0.3};
                            
                        }else{
                            proba = if condicion {0.3} else {0.2};
                        }
                    };
                    //INSERTAMOS LOS PUNTOS Y SUS PROBABILIDADES EN EL HASH MAP
                    opciones.insert(hash.clone() , proba);
                }
                
                
            }

        }
    }
    println!("\n{:?}", opciones);
    

}

