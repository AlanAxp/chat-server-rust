use tokio::net::{TcpListener};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};


// Funcion que me devuelte el valor por default de T
fn give_me_default<T>() -> T  where T: Default {
    Default::default()
}


// Por default Rust no entiende el async en la funcion main, necesitamos la libreria Tokio
// Para poder correr cualquier calculo como async necesitamos un Future (async en JS)

#[tokio::main]
async fn main() {

    // Observando el tipo de retorno, impl Future<Output = Result<TcpListener, Error>>
    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    let (reader, mut writer) = socket.split();

    let mut reader = BufReader::new(reader);
    let mut line = String::new();


    loop {  
        // Todo lo que llegue al socket lo leemos y lo guardamos en el buffer
        let bytes_read  = reader.read_line(&mut line).await.unwrap(); // Regresa el numero de bytes 
        if bytes_read == 0 {
            break;
        }
        // Haremos uso de AsyncWriteExt para escribir en el buffer
        writer.write_all(line.as_bytes()).await.unwrap()
    }
}
