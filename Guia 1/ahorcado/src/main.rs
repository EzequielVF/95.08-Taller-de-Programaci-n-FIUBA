use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::process::exit;
use ahorcado::juego::Juego;

mod juego;

fn leer_y_almacenar_palabras() -> Result<Vec<String>, Error>{
    let mut vector = Vec::new();
    let filename = "src/texto.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        vector.push(line.unwrap());
    }
    Ok(vector)
}

fn jugar(vector: &Vec<String>) {
    let mut var;
    for i in 0..vector.len() {
        let mut juego = Juego::new(vector[i].to_string());
        var = juego.jugar();

        if var.is_err() {
            println!("Perdiste! :,C");
            break;
        } else {
            println!("Pasamos a la siguiente palabra");
        }
    }
}

pub fn leer_vector(vector: &Vec<String>) {
    for i in 0..vector.len() {
        println!("{} - {}", i, vector[i]);
    }
}

fn main()  {
    println!("Bienvenido al ahorcado de FIUBA!");
    let vector = leer_y_almacenar_palabras();
    if vector.is_err() {
        println!("No se pudo leer el archivo!");
        exit(0);
    }
    let vector = vector.unwrap();
    leer_vector(&vector); //Solo es para tener control del contenido del vector
    jugar(&vector);
}