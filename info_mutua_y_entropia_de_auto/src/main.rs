use std::collections::HashMap;

const DIRECCIONES:i32 = 4;
const OPCIONES:i32 = 4;
fn main() {
    //AUTO VIAJA EN ALGUNA DIRECCIÓN. 
    //50% DE LAS VECES SIGUE EN LA MISMA DIRECCIÓN, 
    //30% DA VUELTA A LA DERECHA 
    //20% A LA IZQUIERDA
    
    //CREANOS LA ESTRUCTURA DE DATOS 
    let mut opciones: HashMap<String, f64> = HashMap::new(); 
    let puntos_cardinales: Vec<String> = Vec::<String>::from(["N".to_string(), "E".to_string(), "S".to_string(), "0".to_string()]);
    //LLENAMOS EL HASHMAP
    for i in 0..DIRECCIONES {
        for j in 0..DIRECCIONES {
            //VALIDAMOS QUE NO SE INSERTEN LOS PUNTOS ANTAGONISTAS
            if (i%2) != (j%2) || i==j{
                //println!("{}:{}", puntos_cardinales[i as usize], puntos_cardinales[j as usize]);
                let index:String = format!("{}{}", puntos_cardinales[i as usize], puntos_cardinales[j as usize]);
                opciones.insert(index, 0 as f64);
            }
            // opciones.insert(index, element)
        }
    }
    

}
fn asignar_probabilidad
