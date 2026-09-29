# 📚 Proyecto 20: Packet Analyzer — Documentación Educativa

**Campo:** 🔐 Cybersecurity | **Nivel:** 🟡 Intermedio | **Estado:** ✅ COMPLETADO
**Carpeta:** `cybersecurity/packet_analyzer/`

> **Nota de alcance:** en vez de capturar tráfico real (que requiere privilegios de root y raw sockets vía crates como `pnet`/`pcap`), **construimos los paquetes nosotros mismos como bytes crudos** y escribimos el parser que los decodifica. El concepto central — parsear formatos binarios de red byte a byte — es idéntico, sin necesitar privilegios especiales.

---

## 🎯 Resumen: ¿Qué Construimos?

Un analizador que decodifica headers de red directamente desde bytes: IP (siempre presente), y luego TCP o UDP según lo que indique el propio header IP.

```
Bytes crudos → parse_ip_header → { protocol: 6 (TCP) }
                                        ↓
                            parse_tcp_header(resto de bytes)
                                        ↓
                    "TCP 192.168.1.10:54321 -> 80 (SYN=true, ...)"
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. Extraer bits individuales de un byte con operadores bitwise

```rust
let version = data[0] >> 4;              // primeros 4 bits (nibble alto)
let is_syn = self.flags & 0x02 != 0;      // ¿está encendido el bit 1?
```

- `>> 4` descarta los 4 bits bajos, dejando solo los 4 altos en las posiciones bajas
- `& 0x02` (AND bit a bit) aísla un bit específico: si ese bit está en 1, el resultado es distinto de 0

Estas dos operaciones (shift y mask) son la base de CUALQUIER parsing de formato binario — protocolos de red, formatos de archivo, registros de CPU (ya las usaste en el Proyecto 6.1, paging).

### 2. Combinar 2 bytes en un número de 16 bits (big-endian)

```rust
let src_port = ((data[0] as u16) << 8) | (data[1] as u16);
```

**Diferencia clave con el Proyecto 18:** ahí usamos `to_le_bytes`/`from_le_bytes` (little-endian, nativo de x86). Las redes usan **big-endian** ("network byte order") por convención — el byte MÁS significativo va primero. Por eso aquí se combina manualmente: el primer byte se desplaza 8 posiciones a la izquierda (se vuelve la mitad alta), y se combina con el segundo byte (la mitad baja) usando OR.

### 3. `Vec<u8>` + `extend_from_slice` para construir paquetes por partes

```rust
let mut tcp_full_packet: Vec<u8> = vec![ /* header IP */ ];
tcp_full_packet.extend_from_slice(&[ /* header TCP */ ]);
```

Construimos el "paquete completo" pegando dos arrays de bytes (IP + TCP) en un solo `Vec` — simulando cómo realmente viaja un paquete por la red: un header dentro de otro, todo concatenado en una sola tira de bytes.

### 4. Slicing para separar header de payload

```rust
let payload = &packet[20..]; // todo lo que viene después del header IP
```

El header IP mide exactamente 20 bytes (sin opciones) — `&packet[20..]` toma un slice desde la posición 20 hasta el final, que es exactamente donde empieza el header TCP o UDP.

---

## 🖥️ Conceptos de Redes / Protocolos

### 1. Encapsulación: un protocolo "dentro" de otro

```
Frame Ethernet
  └── Paquete IP
        └── Segmento TCP (o datagrama UDP)
              └── Datos de la aplicación (HTTP, DNS, etc.)
```

Cada capa agrega su propio header ANTES de los datos de la capa superior. Por eso el análisis siempre procede de "afuera hacia adentro": primero IP (para saber protocolo/direcciones), luego TCP/UDP (para saber puertos), luego el payload de aplicación si se quisiera seguir bajando.

### 2. Flags TCP: los "verbos" de una conexión

- **SYN** — "quiero iniciar una conexión" (primer paquete del handshake de 3 vías)
- **ACK** — "confirmo que recibí algo"
- **FIN** — "quiero cerrar la conexión ordenadamente"
- **RST** — "algo salió mal, cierro abruptamente"

Un handshake TCP normal se ve como: `SYN` (cliente) → `SYN+ACK` (servidor) → `ACK` (cliente) — de ahí "three-way handshake". Nuestro paquete de ejemplo (`SYN=true`, resto `false`) simula exactamente el PRIMER paso de ese handshake.

### 3. Por qué UDP no tiene flags

UDP es "sin conexión" — no negocia nada antes de enviar datos, así que no necesita SYN/ACK/FIN. Es más simple y rápido, a costa de no garantizar entrega ni orden (por eso se usa en DNS, streaming, juegos — donde la velocidad importa más que la garantía absoluta de entrega).

---

## 📝 Código Final

Ver `cybersecurity/packet_analyzer/src/main.rs` completo. Estructura:

```rust
struct IpHeader { version: u8, protocol: u8, src_ip: [u8; 4], dst_ip: [u8; 4] }
struct TcpHeader { src_port: u16, dst_port: u16, flags: u8 }
struct UdpHeader { src_port: u16, dst_port: u16, length: u16 }

fn analyze_packet(packet: &[u8]) {
    let ip = parse_ip_header(packet);
    let payload = &packet[20..];
    match ip.protocol {
        6 => { /* parsear como TCP */ }
        17 => { /* parsear como UDP */ }
        _ => { /* protocolo no soportado */ }
    }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| Shift (`>>`) y mask (`&`) | Extraer bits/nibbles específicos | Base de todo parsing binario |
| Big-endian manual | `(byte_alto << 8) \| byte_bajo` | Las redes usan orden distinto a x86 nativo |
| Encapsulación | IP contiene TCP/UDP contiene datos | Cada capa de red agrega su propio header |
| Flags TCP | Bits individuales con significado propio | SYN/ACK/FIN/RST describen el estado de la conexión |
| `&packet[20..]` | Slice para separar header de payload | El tamaño fijo del header IP permite "cortar" ahí |

---

## 🚀 Próximos Pasos

- **Proyecto 21:** Mini IDS (Intrusion Detection System) — usará este mismo parser para detectar patrones sospechosos
- Captura real con `pnet`/`pcap` (requiere configurar permisos, `CAP_NET_RAW` o root)
- Parsear el payload de aplicación (HTTP, DNS) sobre el contenido de TCP/UDP

---

**🎉 Proyecto 20: Completado.**
