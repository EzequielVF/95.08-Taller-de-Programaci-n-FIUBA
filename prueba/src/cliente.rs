//! Se conecta mediante TCP a la dirección asignada por argv.
//! Lee lineas desde stdin y las manda mediante el socket.

use std::env::args;
use std::io::Write;
use std::io::{Read};
use std::net::TcpStream;

static CLIENT_ARGS: usize = 3;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Err(());
    }
    let address = argv[1].clone() + ":" + &argv[2];
    println!("Conectándome a {:?}", address);
    client_run(&address).unwrap();
    Ok(())
}
fn client_run(address: &str) -> std::io::Result<()> {
    let mut socket = TcpStream::connect(address)?;
    let frase = "Hola quiero conectarme".to_string();
    let size_be = (frase.len() as u32).to_be_bytes();
    socket.write(&size_be)?;
    socket.write(&frase.as_bytes())?;


    let mut num_buffer = [0u8; 4];
    socket.read_exact(&mut num_buffer).unwrap();
    let size = u32::from_be_bytes(num_buffer);
    let mut nombre_buf = vec![0; size as usize];
    socket.read_exact(&mut nombre_buf).unwrap();
    let nombre_str = std::str::from_utf8(&nombre_buf).expect("Error al leer nombre");
    println!("Server:{:?}",nombre_str);
    Ok(())
}
