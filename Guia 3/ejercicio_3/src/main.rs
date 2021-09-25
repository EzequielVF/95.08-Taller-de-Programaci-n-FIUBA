use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::thread;
use std::sync::{Arc, mpsc, Mutex};

//Esto deberia ir en Cosa.rs y hacerlo un poco menos acoplado pero para ahorrar tiempo lo dejo asi
pub struct Cosa {
    palabra: String,
    repeticiones: i32,
}
//
fn leer_y_almacenar_palabras(filename :&str) -> HashMap<String, i32>{
    let mut hash: HashMap<String, i32> = HashMap::new();
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

pub fn combinar_hash(aux_1: &mut HashMap<String, i32>, aux_2: &HashMap<String, i32>) {
    for (key, _value) in aux_2.iter() {
        if aux_1.contains_key(key) {
            let aux = aux_1.get(key).unwrap().clone();
            let valor = aux_2.get(key).unwrap().clone();
            let aux = aux + valor;
            aux_1.insert(key.clone(), aux);
        } else {
            let valor = aux_2.get(key).unwrap().clone();
            aux_1.insert(key.clone(), valor);
        }
    }
}
/* // Falta terminar Solucion 2 //
fn main(){
    let (sender, receiver) = mpsc::channel();
    let sender = Arc::new(Mutex::new(sender));

    {
        let mutex_clone = Arc::clone(&sender);
        thread::spawn(move || {
            let hash = leer_y_almacenar_palabras("src/texto.txt");
            mutex_clone.send(hash).unwrap()
        }).join().unwrap();
    }
    {
        let mutex_clone = Arc::clone(&sender);
        thread::spawn(move || {
            let hash = leer_y_almacenar_palabras("src/texto.txt");
            mutex_clone.send(hash).unwrap()
        }).join().unwrap();
    }
    {
        let mutex_clone = Arc::clone(&sender);
        thread::spawn(move || {
            let hash = leer_y_almacenar_palabras("src/texto.txt");
            mutex_clone.send(hash).unwrap()
        }).join().unwrap();
    }
    {
        let mutex_clone = Arc::clone(&sender);
        thread::spawn(move || {
            let hash = leer_y_almacenar_palabras("src/texto.txt");
            mutex_clone.send(hash).unwrap()
        }).join().unwrap();
    }

    let aux = sender.lock().unwrap().recv();
    //
    //
}
*/
/* //Solucion 1//
fn main() {
    let handle_1 = thread::spawn( || {
        let hash = leer_y_almacenar_palabras("src/texto.txt");
        return hash;
    });
    let handle_2 = thread::spawn( || {
        let hash = leer_y_almacenar_palabras("src/texto.txt");
        return hash;
    });
    let handle_3 = thread::spawn( || {
        let hash = leer_y_almacenar_palabras("src/texto.txt");
        return hash;
    });
    let mut aux_1 = handle_1.join().unwrap();
    let aux_2 = handle_2.join().unwrap();
    let aux_3 = handle_3.join().unwrap();

    combinar_hash(&mut aux_1, &aux_2);
    combinar_hash(&mut aux_1, &aux_3);
    let mut vec = Vec::new();
    llenar_vector(&mut vec, &aux_1);
    ordenar_vector(&mut vec);
    leer_vector(&vec);
}
*/