
use std::{io::stdin as mystdin};
fn main() {
    //let abc:Vec<String> = Vec::from(["0".to_string(), "1".to_string()]);
    let abc: Vec<String> = leer_abecedario();
    let n: i32 = abc.len() as i32;
    let r: i32 = pedir_numero();
    
    //HAY QUE GENERAR LAS COMBINACIONES DEL ABECEDARIO DE ENTRADA    
    for len in 1..r+1{
        
        let mut r: i32 = len;
        let mut i: i32 = 0;
        let mut indexs: Vec<usize> = Vec::new();
        combinatoria(&abc, n, &mut r, &mut i, &mut indexs);
        
    }
    
}
fn combinatoria(abc: &Vec<String>, n: i32, r: &mut i32, i: &mut i32, indexs: &mut Vec<usize>){
    if indexs.len() == 0 {
        for _r_i in 0..*r{
            indexs.push(0 as usize);
        }
    }

    let r = r;
    if i < r {
        for x in 0..n{
            indexs[*i as usize]=x as usize;
            let mut pendejada = *i+1;
            combinatoria(&abc, n, r, &mut pendejada, indexs)
        }
    }
    else {
        println!("");
        for w in 0..*r{
            print!("{}|", abc[indexs[w as usize]]);
        }
    }
}
fn pedir_numero()->i32{
    loop{
        let mut numero_s: String = String::new();
        println!("Ingresa la longitud máxima del mensaje:");
        mystdin().read_line(&mut numero_s).expect("Fail to read");
        let numero_i: i32 = match numero_s.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        if numero_i != 0 { break numero_i }
    }
}
fn leer_abecedario() -> Vec<String>{
    loop{
        let mut abc: String = String::new();
        println!("Ingrese el abecedario separando los items por comas: ");
        mystdin().read_line(&mut abc).expect("Failed to read");
        
        let abc = abc.trim();
                
        if abc.len() == 0 {
            continue;
        }

        let items: Vec<&str> = abc.split(",").collect();

        let mut abecedario: Vec<String> = Vec::new();
        for item in items{
            abecedario.push(String::from(item.trim()));
        }
        break abecedario
    }
}
fn _calcular_factorial(n: i32) -> i32{
    if n == 0 {
        1
    }
    else {
        n*_calcular_factorial(n-1)
    }
}