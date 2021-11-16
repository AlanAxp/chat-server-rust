use tokio::net::{TcpListener};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::broadcast;


// Funcion que me devuelte el valor por default de T
// fn give_me_default<T>() -> T  where T: Default {
//     Default::default()
// }
// Turbo fish operator
// let value = give_me_default::<i32>();


// Por default Rust no entiende el async en la funcion main, necesitamos la libreria Tokio
// Para poder correr cualquier calculo como async necesitamos un Future (async en JS)

#[tokio::main]
async fn main() {

    // Observando el tipo de retorno, impl Future<Output = Result<TcpListener, Error>>
    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    // Crearemos un canal que nos permitira hacer que multiples clientes puedan leer a los demas.
    // Ya no va ser un string pues hemos anadido que sea una tupla
    let (tx, _rx) = broadcast::channel(10 /*la cantidad de conecciones permitidas */);

        
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        
        // Para manejar una tarea que puede ser manejada por multiples clientes
        tokio::spawn( async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {  
                // Agregamos el macro que nos permitira correr multiples cosas async al mismo tiempo
                tokio::select! {
                    
                    // Todo lo que llegue al socket lo leemos y lo guardamos en el buffer
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        // Para poder recibir mensajes en el canal
                        // Agregamos la direccion de donde llego para mandarla al mismo sitio
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            // Haremos uso de AsyncWriteExt para escribir en el buffer
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}