# 📚 Proyecto 7: TCP Client — Documentación Educativa

**Campo:** 🌐 Networking | **Nivel:** 🟢 Básico | **Estado:** ✅ COMPLETADO
**Carpeta:** `networking/tcp_client/`

---

## 🎯 Resumen: ¿Qué Construimos?

Un cliente TCP que se conecta a un servidor real, envía una petición HTTP a mano (texto plano) y lee la respuesta completa.

```
tcp_client example.com:80
  ↓
TcpStream::connect
  ↓
write_all("GET / HTTP/1.1...")
  ↓
read_to_string(&mut response)
  ↓
Imprime la respuesta HTTP completa
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `TcpStream::connect` y `Result`

```rust
match TcpStream::connect(address) {
    Ok(mut stream) => { /* usar stream */ }
    Err(e) => println!("Error al conectar: {}", e),
}
```

**¿Por qué `Result`?**
- Conectar por red puede fallar (host caído, sin internet, puerto cerrado)
- Rust obliga a manejar ambos casos explícitamente — no hay "excepciones silenciosas"

### 2. Los traits `Read` y `Write`

```rust
use std::io::{Read, Write};

stream.write_all(request.as_bytes()).unwrap();  // Write
stream.read_to_string(&mut response).unwrap();  // Read
```

**¿Qué son?**
- `Write` = "este tipo puede recibir bytes" (archivos, sockets, stdout...)
- `Read` = "este tipo puede producir bytes"
- `TcpStream` implementa AMBOS — por eso se puede enviar y recibir con la misma conexión

**Ventaja de los traits:** el mismo código (`write_all`, `read_to_string`) funciona igual para un archivo, un socket TCP, o cualquier otro tipo que implemente esos traits.

### 3. `env::args()` — argumentos de línea de comandos

```rust
let args: Vec<String> = env::args().collect();
let address = &args[1];
```

`args[0]` siempre es el nombre del binario ejecutado; los argumentos reales empiezan en `args[1]`.

---

## 🖥️ Conceptos de Networking

### 1. TCP: conexión antes de datos

TCP (a diferencia de UDP) requiere establecer una conexión (handshake) antes de poder enviar datos. `TcpStream::connect()` hace ese handshake por nosotros.

### 2. HTTP es texto plano sobre TCP

```
GET / HTTP/1.1\r\n
Host: example.com\r\n
Connection: close\r\n
\r\n
```

- Cada línea termina en `\r\n` (retorno de carro + salto de línea, no solo `\n`)
- Una línea vacía separa los headers del cuerpo (aquí no hay cuerpo en un GET)
- `Connection: close` le pide al servidor que cierre la conexión al terminar — así `read_to_string` sabe cuándo parar (llega EOF)

### 3. Chunked Transfer Encoding

En la respuesta real viste:
```
Transfer-Encoding: chunked
...
22f
<html>...
0
```
El servidor envía el cuerpo en "trozos" (`22f` = tamaño del trozo en hexadecimal), terminando con un trozo de tamaño `0`. Esto permite enviar contenido sin saber su tamaño total de antemano. (Nuestro cliente no lo parsea, solo lo muestra crudo — un parser HTTP real sí lo interpretaría.)

---

## 📝 Código Final

```rust
use std::net::TcpStream;
use std::io::{Read, Write};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: tcp_client <host:puerto>");
        return;
    }

    let address = &args[1];

    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Conectado a {}!", address);

            let host = address.split(':').next().unwrap_or(address);
            let request = format!(
                "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                host
            );
            stream.write_all(request.as_bytes()).unwrap();
            println!("Petición enviada.");

            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            println!("--- Respuesta ---");
            println!("{}", response);
        }
        Err(e) => println!("Error al conectar: {}", e),
    }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `TcpStream::connect` | Abre conexión TCP | Punto de entrada de todo networking en Rust |
| `Result<T, E>` | Éxito o error explícito | La red siempre puede fallar |
| `Read` / `Write` traits | Interfaz genérica de I/O | Mismo código sirve para sockets, archivos, etc. |
| `Connection: close` | Header HTTP | Permite saber cuándo termina la respuesta |

---

## 🚀 Próximos Pasos

- **Proyecto 8:** HTTP Server (el lado servidor de esta misma conexión)
- Parsear la respuesta HTTP en vez de imprimirla cruda
- Manejar HTTPS (requiere TLS, más complejo)

---

**🎉 Proyecto 7: Completado. Primer proyecto del campo Networking.**
