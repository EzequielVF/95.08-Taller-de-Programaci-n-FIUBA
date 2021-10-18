fn main(){
    let numero = 25;
    let aux = numero.to_be_bytes();
    let numero_aux = u32::from_be_bytes(aux);
}