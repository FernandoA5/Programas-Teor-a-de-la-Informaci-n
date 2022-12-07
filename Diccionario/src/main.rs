pub mod lector;

use lector::Texto;
#[derive(Debug, Clone)]
struct Referencia{
     offset: i32,
     len: i32,
     character: char
}
fn main() {     
     const VENTANA: i32 = 21;
     const FUTURO: i32 = 12;
     let mut texto: Texto = Texto {
          nombre: "src/letra.txt".to_string(),
          contenido: "".to_string()
     };
     texto.leer_archivo();
     let caracteres: Vec<char> = texto.contenido.chars().collect();
     let mut exit=false;
     let mut contador_avance=0;
     let mut lista_referencias:Vec<Referencia>=Vec::new();
     let mut lista_textos: Vec<String>=Vec::new();
     let mut iteraciones: i32 = 0;
     while exit==false{
          iteraciones+=1;
          let mut buffer: String = String::new();
          let mut buffer_aux: String= String::new();
          let mut ref_match: Referencia=Referencia{offset: 0,len: 0,character: '0'};
          let mut ref_match_aux: Referencia=Referencia{offset: 0,len: 0,character: '0'};
          for i in contador_avance..VENTANA+contador_avance{ //HISTORIAL
               print!("[{}]", caracteres[i as usize]);
          }
          print!(" || ");
          for i in contador_avance+VENTANA..contador_avance+VENTANA+FUTURO{ //LOOKAHEAD
               print!("[{}]", caracteres[i as usize]);
          }
          print!("\n");
          //RECORREMOS TODOS LOS ELEMENTOS DEL LOOKAHEAD
          for j in contador_avance+VENTANA..contador_avance+VENTANA+FUTURO{
               let mut cont_offset=0;
               for k in contador_avance..VENTANA+contador_avance{ //LOS COMPARAMOS CON EL HISTORIAL
                    cont_offset+=1;
                    let mut contador_len=0;
                    if caracteres[j as usize]==caracteres[k as usize]{ //IF ENCUENTRA UNA COINCIDENCIA
                         //DEBEMOS RECORRER DESDE AQUÍ HACIA ADELANTE EN BUSCA DE MÁS COINCIDENCIAS PARA J
                         let mut buffer_aux:String = String::new();
                         let mut j_temp=j;
                         for l in k..VENTANA+contador_avance{
                              let cond_iguales= caracteres[j_temp as usize] == caracteres[l as usize];
                              if cond_iguales{
                                   buffer_aux.push(caracteres[j_temp as usize]);
                                   contador_len+=1;
                              }
                              j_temp+=1;
                              let cond_limite=j_temp-j+cont_offset >= FUTURO-2;
                              if !cond_iguales ||  cond_limite{
                                   if buffer_aux.len()>buffer.len(){
                                        buffer=buffer_aux.clone();
                                        ref_match=Referencia{
                                             offset:VENTANA-cont_offset+1, len: buffer.len() as i32,
                                             character: caracteres[(j_temp-iteraciones) as usize]
                                        };
                                   }
                                   break;
                              }
                         }
                    }                 
               }
          }
          if buffer_aux.len()==0 && buffer.len() == 0{ //GUARDAMOS EL PRIMER CARACTER DEL LOOKAHEAD
               buffer_aux.push(caracteres[(contador_avance+VENTANA) as usize]); //GUARDAMOS EL CARACTER
               ref_match_aux=Referencia{offset: 1, len: 1, character: caracteres[(contador_avance+VENTANA) as usize]
               };
               lista_referencias.push(ref_match_aux.clone());
               lista_textos.push(buffer_aux.clone());
          }else{
               lista_referencias.push(ref_match.clone());
               contador_avance = if (contador_avance+(buffer.len()-1) as i32) <= caracteres.len() as i32{
                    (contador_avance + (buffer.len() as usize) as i32) - iteraciones
               }else{break};
               lista_textos.push(buffer.clone());
          }
          println!("Ref: {:?}", lista_referencias);
          println!("texts: {:?}", lista_textos);
          print!("\n");
          contador_avance+=1;
          exit = if contador_avance+VENTANA+FUTURO == caracteres.len() as i32 { true } else { false };
          exit = if iteraciones==5 {true} else{false};
     }
     
}

