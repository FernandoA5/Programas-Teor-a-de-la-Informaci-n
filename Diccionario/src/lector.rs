use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Texto{
    pub nombre: String,
    pub contenido: String
}

impl Texto{
    pub fn leer_archivo(&mut self){
        let archivo: File = match File::open(&self.nombre){
            Ok(file) => file,
            Err(err) => {
                println!("Error: {}", err);
                std::process::exit(1);
            }
        };
        println!("{:?}", archivo);
        let mut buf_reader = BufReader::new(archivo);
        let mut contenido: String = String::new();
        buf_reader.read_to_string(&mut contenido).unwrap();

        let contenido_limpio: String = Self::limpiar_contenido(contenido);
        self.contenido=contenido_limpio;

    }
    pub fn limpiar_contenido(contenido: String) -> String{
        let lineas = contenido.lines();
        let mut contenido_limpio: String = String::new();
        for linea in lineas{
            contenido_limpio.push_str(linea);
            contenido_limpio.push_str(" ");
        }
        contenido_limpio
    }
}