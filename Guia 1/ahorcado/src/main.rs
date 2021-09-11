use std::fs::File;
use std::io::{BufRead, BufReader};
use ahorcado::juego::Juego;

mod juego;

fn leer_y_almacenar_palabras() -> Vec<String>{
    let mut vector = Vec::new();
    let filename = "src/texto.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        vector.push(line.unwrap());
    }
    vector
}

fn jugar(vector: &Vec<String>) {
    let mut var:i8 = 0;
    for i in 0..vector.len() {
        let mut juego = Juego::new(vector[i].to_string());
        var = juego.jugar();

        if var > 0 {
            println!("Pasamos a la siguiente palabra");
        } else {
            println!("Perdiste! :,C");
            break;
        }
    }
}

fn leer_vector(vector: &Vec<String>) {
    for i in 0..vector.len() {
        println!("{} - {}", i, vector[i]);
    }
}

fn main()  {
    println!("Bienvenido al ahorcado de FIUBA!");
    let vector = leer_y_almacenar_palabras();
    leer_vector(&vector); //Solo es para tener control del contenido del vector
    jugar(&vector);
}