use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn leer_y_almacenar_stopwords() -> HashMap<String, i32>{
    let mut hash: HashMap<String, i32> = HashMap::new();
    let filename = "src/stopwords.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        hash.insert(line.unwrap(), 1);
    }
    hash
}

fn es_stopword(stop_words: &HashMap<String, i32>, palabra :&str) -> bool{
    if stop_words.contains_key(palabra) {
        true
    } else {
        false
    }
}

fn recorrer_archivo_y_almacenar(indice: &mut HashMap<String, HashMap<String, i32>>, stop_words: &HashMap<String, i32>, filename: &str, id: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let oracion: String = line.unwrap().to_lowercase().into();
        let chunks: Vec<_> = oracion.split_whitespace().collect();
        for palabra in chunks {
            if !es_stopword(stop_words, palabra) { // Si no es una stopword lo agrego
                if indice.contains_key(&palabra.to_string()) { //Si la palabra ya esta en el indice
                    let mut aux = indice.get(&palabra.to_string()).unwrap().clone();
                    if aux.contains_key(&id.to_string()) { //Ya aparecio esta palabra en este documento antes
                        let valor = aux.get(&id.to_string()).unwrap();
                        let valor = valor + 1;
                        aux.insert(id.to_string(), valor); // Actualizo el valor de apariciones en el hash
                        indice.insert(palabra.to_string(), aux); //Actualizo el hash asociado a esa clave
                    } else { //La palabra no aparecio previamente en este documento
                        aux.insert(id.to_string(), 1);
                        indice.insert(palabra.to_string(), aux);
                    }
                } else { // Si no estaba todavia en el indice
                    let mut hash_aux: HashMap<String, i32> = HashMap::new();
                    hash_aux.insert(id.to_string(), 1);
                    indice.insert(palabra.to_string(), hash_aux);
                }
            }
        }
    }
}

fn leer_y_llenar_indice(stop_words :&HashMap<String, i32>) -> HashMap<String, HashMap<String, i32>>{
    let mut indice: HashMap<String, HashMap<String, i32>> = HashMap::new();
    recorrer_archivo_y_almacenar(&mut indice, stop_words,"src/texto1.txt", "1");
    recorrer_archivo_y_almacenar(&mut indice, stop_words,"src/texto2.txt", "2");
    recorrer_archivo_y_almacenar(&mut indice, stop_words,"src/texto3.txt", "3");
    indice
}

fn leer_hash(hash: &HashMap<String, i32>) {
    for (key, value) in hash.iter() {
        println!("{} - {}", key, value);
    }
}

fn recorrer_indice(indice: &HashMap<String, HashMap<String, i32>>) {
    for (key, hash) in indice.iter() {
        println!("La palabra:{}", key);
        println!("Aparece en:");
        for (id, cantidad) in hash.iter() {
            println!("En el Documento -> {},  -> {} veces.", id, cantidad);
            println!();
        }
    }
}

fn pedir_y_armar_hash_con_frase_para_busqueda(stop_words :&HashMap<String, i32>) -> HashMap<String, i32>{
    let mut hash: HashMap<String, i32> = HashMap::new();
    let mut p = String::new();
    println!("Ingresa la frase a buscar:");
    println!();
    std::io::stdin().read_line(&mut p).unwrap();
    let p = p.to_string().to_lowercase();
    let palabras: Vec<_> = p.split_whitespace().collect();
    for palabra in palabras {
        if !es_stopword(stop_words, palabra) {
            if !hash.contains_key(&palabra.to_string()) {
                hash.insert(palabra.to_string(), 1);
            }
        }
    }
    hash
}

fn buscar_en_indice(indice: &HashMap<String, HashMap<String, i32>>, stop_words :&HashMap<String, i32>) {
    let frase = pedir_y_armar_hash_con_frase_para_busqueda(stop_words);
    let mut resultado :HashMap<String, f32> = HashMap::new();
    resultado.insert("1".to_string(), 0.0);
    resultado.insert("2".to_string(), 0.0);
    resultado.insert("3".to_string(), 0.0);
    for (palabra, _nada) in frase.iter() { //Voy viendo palabra a palabra
        if indice.contains_key(palabra) { // Si la palabra esta en el indice
            let mut aux = indice.get(palabra).unwrap().clone();
            // Mirar en cuantos documentos aparece
            // Calcular el log
            // Asignar los resultados en el hash para resultados
        }
    }
    // Fijarse cual de los documentos tiene el mayor "puntaje"
}

fn main() {
    let stop_words = leer_y_almacenar_stopwords(); //Me guardo las palabras que no deberia almacenar
    //leer_hash(&stop_words); //Muestra la lista de stop-words
    let indice :HashMap<String, HashMap<String, i32>> = leer_y_llenar_indice(&stop_words);
    recorrer_indice(&indice);
    buscar_en_indice(&indice, &stop_words);
}
