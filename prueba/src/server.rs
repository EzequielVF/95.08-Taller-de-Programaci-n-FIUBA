use std::env::args;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;
use std::io::{BufReader, BufRead, Read};

static SERVER_ARGS: usize = 2;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Err(());
    }

    let address = "0.0.0.0:".to_owned() + &argv[1];

    server_run(&address).unwrap();
    Ok(())
}

fn server_run(address: &str) -> std::io::Result<()> {
    let mut id = 0;
    loop {
        let listener = TcpListener::bind(address)?;
        let connection = listener.accept()?;
        let mut client_stream : TcpStream = connection.0;
        thread::spawn(move || {
            handle_client(&mut client_stream, &id);
        });
        id = id + 1;
    }
}

fn handle_client(stream: &mut TcpStream, id: &i32) {
    let mut num_buffer = [0u8; 4];
    stream.read_exact(&mut num_buffer).unwrap();
    let size = u32::from_be_bytes(num_buffer);
    let mut nombre_buf = vec![0; size as usize];
    stream.read_exact(&mut nombre_buf).unwrap();
    let nombre_str = std::str::from_utf8(&nombre_buf).expect("Error al leer nombre");
    println!("Cliente -{:?}-: {:?}", id, nombre_str);


    let mut frase = "Te conectaste exitosamente".to_string();
    let size_be = (frase.len() as u32).to_be_bytes();
    stream.write(&size_be).unwrap();
    stream.write(&frase.as_bytes()).unwrap();
    println!("Envie una respuesta al cliente {:?}", id);
}
