# Motivación

El objetivo de este código es entender el funcionamiento de los websockets creando una aplicación simple de chat.

- Uso de la libreria Tokio para las operaciones async 
- Creación de un servidor para chat
    Basicamente es permitir la conección de multiples Tcp clients al servidor del chat y cuando un cliente envia un mensaje al server el mensaje será transmitido (broadcasted) a todos los demás clientes que estan conectados al servidor.


# Compilación

1. Clonar el repositorio.

2. Ejecutar.

```bash
cargo run
```

3. En otras terminales usar telnet como clinte.

```bash
telnet localhost 8000
```