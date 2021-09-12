/// #TDA Juego
#[derive(Debug)]
pub struct Juego {
    palabra: String,
    longitud: usize,
    intentos: i8,
}
#[derive(Debug)]
#[warn(dead_code)]
pub enum Errores {
    SinMovimientos,
}

#[allow(dead_code)]
impl Juego {
    /// Constructor

    pub fn new(palabra: String) -> Self {
        return Juego {palabra: palabra.to_lowercase(), longitud: palabra.len(), intentos: 5}
    }

    fn mostrar_info(&self, vector: &Vec<bool>, string_indexeable: &Vec<char>, adivinadas: &mut Vec<char>, equivocadas: &mut Vec<char>) {
        print!("La palabra hasta el momento es: ");

        for i in 0..string_indexeable.len() {
            if vector[i] == false {
                print!("_");
            } else {
                print!("{}", string_indexeable[i]);
            }
        }
        println!();
        print!("Adivinaste las siguientes letras: ");
        for letra in adivinadas {
            print!("{}-", letra);
        }
        println!();
        print!("Las siguientes letras que ingresaste no fueron correcta: ");
        for letra in equivocadas {
            print!("{}-", letra);
        }
        println!();
        println!("Te quedan {} intentos.", self.intentos);
        println!("Ingresa una letra: ");
    }

    fn solicitar_letra(&mut self, vector: &mut Vec<bool>, string_indexeable: &Vec<char>, adivinadas: &mut Vec<char>, equivocadas: &mut Vec<char>) -> Result<(), Errores> {
        let mut p = String::new();
        let mut valida = false;
        let mut invalida_repetida = false;
        std::io::stdin().read_line(&mut p).unwrap();
        let p = p.to_string().to_lowercase();
        let char_vec: Vec<char> = p.chars().collect();

        println!("La letra ingresada es -> {}", p);
        for i in 0..string_indexeable.len() {
            if string_indexeable[i] == char_vec[0] {
                if !vector[i] {
                    adivinadas.push(char_vec[0]);
                    vector[i] = true;
                } else {
                    println!("Ya habias ingresado esta letra y era correcta!.");
                    println!();
                }
                valida = true;
            }
        }
        if !valida {
            for letra in &*equivocadas{
                let aux = letra.clone();
                if aux == char_vec[0]{
                    invalida_repetida = true;
                    println!("Letra erronea ya ingresada.");
                    println!();
                }
            }
            if !invalida_repetida {
                println!("Letra erronea");
                equivocadas.push(char_vec[0]);
            }
            self.intentos -= 1;
        }
        Ok(())
    }

    fn adivino_todas(vector: &Vec<bool>) -> bool {
        for i in 0..vector.len() {
            if !vector[i] {
                return false;
            }
        }
        true
    }

    pub fn jugar(&mut self) -> Result<i8, Errores> {
        let mut vector = Vec::new();
        let mut adivinadas :Vec<char> = Vec::new();
        let mut equivocadas :Vec<char> = Vec::new();
        let string_indexeable: Vec<char> = self.palabra.chars().collect();
        for _i in 0..self.palabra.len() { //Lleno el vector de false
            vector.push(false);
        }
        while self.intentos > 0 {
            self.mostrar_info(&vector, &string_indexeable, &mut adivinadas, &mut equivocadas);
            self.solicitar_letra(&mut vector, &string_indexeable, &mut adivinadas, &mut equivocadas)?;
            if Juego::adivino_todas(&vector) {
                println!("Felicidades!! Adivinaste la palabra, esta era -> {}", self.palabra);
                println!();
                println!();
                break;
            }
        } //No es lo mas optimo pero tenia q agregar el sistema de errores
        if self.intentos < 1 {
            Err(Errores::SinMovimientos)
        } else {
            Ok(self.intentos)
        }
    }
}