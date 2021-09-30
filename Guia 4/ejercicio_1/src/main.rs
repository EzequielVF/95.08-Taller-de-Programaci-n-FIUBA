use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, BufRead, Write};
/////////////////////////////////////////////////////////////////////////////////////
fn server_run(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    let connection = listener.accept()?;
    let mut client_stream : TcpStream = connection.0;

    let line = "Hola Fran".to_string();
    client_stream.write(line.as_bytes())?;
    client_stream.write("\n".as_bytes())?;

    handle_client(&mut client_stream)?;

    let address_aux = "0.0.0.0:".to_owned() + &"4667".to_string();
    let listener = TcpListener::bind(address_aux)?;
    let connection = listener.accept()?;
    let mut client_stream : TcpStream = connection.0;

    let line = "Hola Mati".to_string();
    client_stream.write(line.as_bytes())?;
    client_stream.write("\n".as_bytes())?;

    handle_client(&mut client_stream)?;
    Ok(())
}

fn handle_client(stream: &mut dyn Read) -> std::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
        println!("{:?}", line.unwrap());
        break; //Muy random la verdad
    }
    Ok(())
}
/////////////////////////////////////////////////////////////////////////////////////
fn client_run(address: &str) -> std::io::Result<()> {
    let mut socket = TcpStream::connect(address)?;
    handle_client(&mut socket);
    let line = "Buen día Papá".to_string();
    socket.write(line.as_bytes())?;
    socket.write("\n".as_bytes())?;
    Ok(())
}

fn main() {
    thread::spawn(move || {
        let address = "0.0.0.0:".to_owned() + &"4666".to_string();
        server_run(&address).unwrap();
    });
    thread::spawn(move || {
        let address = "127.0.0.1".to_owned() + ":" + &"4666".to_string();
        println!("Conectándome a {:?}", address);
        client_run(&address).unwrap();
    }).join().unwrap();

    thread::spawn(move || {
        let address = "127.0.0.1".to_owned() + ":" + &"4667".to_string();
        println!("Conectándome a {:?}", address);
        client_run(&address).unwrap();
    }).join().unwrap();
}
