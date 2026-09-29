#[derive(Debug)]
struct IpHeader {
    version: u8,
    protocol: u8,
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
}

#[derive(Debug)]
struct TcpHeader {
    src_port: u16,
    dst_port: u16,
    flags: u8,
}

#[derive(Debug)]
struct UdpHeader {
    src_port: u16,
    dst_port: u16,
    length: u16,
}

impl TcpHeader {
    fn is_syn(&self) -> bool { self.flags & 0x02 != 0 }
    fn is_ack(&self) -> bool { self.flags & 0x10 != 0 }
    fn is_fin(&self) -> bool { self.flags & 0x01 != 0 }
    fn is_rst(&self) -> bool { self.flags & 0x04 != 0 }
}

fn parse_ip_header(data: &[u8]) -> IpHeader {
    let version = data[0] >> 4;     // primeros 4 bits del byte 0
    let protocol = data[9];
    let src_ip = [data[12], data[13], data[14], data[15]];
    let dst_ip = [data[16], data[17], data[18], data[19]];

    IpHeader { version, protocol, src_ip, dst_ip }
}

fn parse_tcp_header(data: &[u8]) -> TcpHeader {
    let src_port = ((data[0] as u16) << 8) | (data[1] as u16);
    let dst_port = ((data[2] as u16) << 8) | (data[3] as u16);
    let flags = data[13];

    TcpHeader { src_port, dst_port, flags }
}

fn parse_udp_header(data: &[u8]) -> UdpHeader {
    let src_port = ((data[0] as u16) << 8) | (data[1] as u16);
    let dst_port = ((data[2] as u16) << 8) | (data[3] as u16);
    let length = ((data[4] as u16) << 8) | (data[5] as u16);

    UdpHeader { src_port, dst_port, length }
}

fn protocol_name(protocol: u8) -> &'static str {
    match protocol {
        6 => "TCP",
        17 => "UDP",
        1 => "ICMP",
        _ => "desconocido",
    }
}

fn analyze_packet(packet: &[u8]) {
    let ip = parse_ip_header(packet);
    println!("IP: {}.{}.{}.{} -> {}.{}.{}.{} ({})",
        ip.src_ip[0], ip.src_ip[1], ip.src_ip[2], ip.src_ip[3],
        ip.dst_ip[0], ip.dst_ip[1], ip.dst_ip[2], ip.dst_ip[3],
        protocol_name(ip.protocol)
    );

    let payload = &packet[20..];    // el header IP mide 20 bytes (sin opciones)

    match ip.protocol {
        6 => {
            let tcp = parse_tcp_header(payload);
            println!("  TCP {}:{} -> {} (SYN={}, ACK={}, FIN={}, RST={})",
                ip_str(&ip.src_ip), tcp.src_port, tcp.dst_port,
                tcp.is_syn(), tcp.is_ack(), tcp.is_fin(), tcp.is_rst());
        }
        17 => {
            let udp = parse_udp_header(payload);
            println!(" UDP {}:{} -> {} (length={})",
                ip_str(&ip.src_ip), udp.src_port, udp.dst_port, udp.length);
        }
        _ => println!(" (protocolo no soportado para parseo detallado)"),
    }
}

fn ip_str(ip: &[u8; 4]) -> String {
    format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
}

fn main() {
    // Contruimos un paquete IP+TCP completo (IP header + TCP header pegados)
    let mut tcp_full_packet: Vec<u8> = vec![
        0x45, 0x00, 0x00, 0x28, 0x00, 0x00, 0x40, 0x00, 0x40, 0x06, 0x00, 0x00,
        192, 168, 1, 10, 192, 168, 1, 20,
    ];
    tcp_full_packet.extend_from_slice(&[
        0xD4, 0x31, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x50, 0x02, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ]);

    let mut udp_full_packet: Vec<u8> = vec![
        0x45, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11, 0x00, 0x00,
        192, 168, 1, 10, 192, 168, 1, 20,
    ];
    udp_full_packet.extend_from_slice(&[0x14, 0xE9, 0x14, 0xE9, 0x00, 0x10, 0x00, 0x00]);

    println!("Analizando paquete 1.");
    analyze_packet(&tcp_full_packet);

    println!("\nAnalizando paquete 2:");
    analyze_packet(&udp_full_packet);
}