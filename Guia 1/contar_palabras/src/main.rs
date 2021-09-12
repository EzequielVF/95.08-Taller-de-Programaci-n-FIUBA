use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

//Esto deberia ir en Cosa.rs y hacerlo un poco menos acoplado pero para ahorrar tiempo lo dejo asi
pub struct Cosa {
    palabra: String,
    repeticiones: i32,
}
//
fn leer_y_almacenar_palabras() -> HashMap<String, i32>{
    let mut hash: HashMap<String, i32> = HashMap::new();

    let filename = "src/texto.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let oracion: String = line.unwrap().to_lowercase().into();
        let chunks: Vec<_> = oracion.split_whitespace().collect();
        for word in chunks {
            if hash.contains_key(&word.to_string()) {
                let valor = hash.get(&word.to_string()).unwrap();
                let valor = valor + 1;
                hash.insert(word.to_string(), valor);
            } else {
                hash.insert(word.to_string(), 1);
            }
        }
    }
    hash
}

pub fn llenar_vector(vector: &mut Vec<Cosa>, hash: &HashMap<String, i32>) {
    for (key, value) in hash.iter() {
        vector.push(Cosa {palabra:key.clone(), repeticiones: value.clone()});
    }
}

pub fn ordenar_vector(vector: &mut Vec<Cosa>) {
    vector.sort_by(|a, b| b.repeticiones.cmp(&a.repeticiones))
}

/*pub fn leer_hash(hash: &HashMap<String, i32>) {
    for (key, value) in hash.iter() {
        println!("{} - {}", key, value);
    }
}*/

pub fn leer_vector(vector: &Vec<Cosa>) {
    for i in 0..vector.len() {
        println!("{} -> {}", vector[i].palabra, vector[i].repeticiones);
    }
}

fn main() {
    let hash = leer_y_almacenar_palabras();
    //leer_hash(&hash); //Solo es para tener control del contenido del hash
    let mut vec = Vec::new();
    llenar_vector(&mut vec, &hash);
    ordenar_vector(&mut vec);
    leer_vector(&vec); //Solo es para tener control del contenido del vector
}
