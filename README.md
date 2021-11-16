El objetivo de este código es entender el funcionamiento de los websockets creando una aplicación simple de chat.

- Uso de la libreria Tokio para las operaciones async 
- Creación de un servidor para chat
    Basicamente es permitir la conección de multiples Tcp clients al servidor del chat y cuando un cliente envia un mensaje al server el mensaje será transmitido (broadcasted) a todos los demás clientes que estan conectados al servidor.

Proyecto movido a /Rust-codes/
