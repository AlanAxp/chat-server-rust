use tokio::{io::{AsyncBufRead, AsyncReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::broadcast};

// Funcion que me devuelte el valor por default de T
fn give_me_default<T>() -> T  where T: Default {
    Default::default()
}


// Por default Rust no entiende el async en la funcion main, necesitamos la libreria Tokio
// Para poder correr cualquier calculo como async necesitamos un Future (async en JS)

#[tokio::main]
async fn main() {

    // Observando el tipo de retorno, impl Future<Output = Result<TcpListener, Error>>
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {// Creamos un buffer de 1KB para guardar los mensajes entrantes
        let mut buffer = [0u8; 1024];

        // Todo lo que llegue al socket lo leemos y lo guardamos en el buffer
        let bytes_read  = socket.read(&mut buffer).await.unwrap(); // Regresa el numero de bytes 

        // Haremos uso de AsyncWriteExt para escribir en el buffer
        socket.write_all(&buffer[..bytes_read]).await.unwrap()
    }
}
