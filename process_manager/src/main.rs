use std::fs;

fn obtener_nombre_proceso(pid: &str) -> Option<String> {
    let status_path = format!("/proc/{}/status", pid);
    let contenido = fs::read_to_string(&status_path).ok()?;

    for linea in contenido.lines() {
        if linea.starts_with("Name:") {
            let nombre = linea.strip_prefix("Name:")?.trim();
            return Some(nombre.to_string());
        }
    }
    None
}

fn obtener_memoria_proceso(pid: &str) -> Option<u64> {
    let status_path = format!("/proc/{}/status", pid);
    let contenido = fs::read_to_string(&status_path).ok()?;

    for linea in contenido.lines() {
        if linea.starts_with("VmRSS:") {
            // VmRSS está en kilobytes, vamos a extraerlo
            let valor_str = linea.strip_prefix("VmRSS:")?.trim();
            let valor_str = valor_str.split_whitespace().next()?;
            let kilobytes: u64 = valor_str.parse().ok()?;
            return Some(kilobytes * 1024); // Convertir a bytes
        }
    }
    None
}

fn obtener_cpu_proceso(pid: &str) -> Option<f64> {
    let stat_path = format!("/proc/{}/stat", pid);
    let contenido = fs::read_to_string(&stat_path).ok()?;

    let campos: Vec<&str> = contenido.split_whitespace().collect();

    if campos.len() < 15 {
        return None;
    }

    let utime: u64 = campos[13].parse().ok()?;
    let stime: u64 = campos[14].parse().ok()?;

    let total_ticks = utime + stime;

    Some(total_ticks as f64 / 100.0)
}

fn main() {
    println!("Listando procesos del sistema...\n");
    
    let proc_path = std::path::Path::new("/proc");
    
    match fs::read_dir(proc_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let file_name = entry.file_name();
                    let name = file_name.to_string_lossy().to_string();
                    
                    if name.chars().all(|c| c.is_digit(10)) {
                        if let Some(nombre) = obtener_nombre_proceso(&name) {
                            let memoria = obtener_memoria_proceso(&name)
                                .unwrap_or(0);
                            let memoria_mb = memoria as f64 / (1024.0 * 1024.0);
                            
                            let cpu = obtener_cpu_proceso(&name)
                                .unwrap_or(0.0);

                            println!("{:<6} {:<20} {:<10} {:.1}%", 
                                name, nombre, 
                                format!("{:.1} MB", memoria_mb),
                                cpu);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error leyendo /proc: {}", e);
        }
    }
}
