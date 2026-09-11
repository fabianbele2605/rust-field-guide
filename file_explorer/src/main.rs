use std::fs;

fn formato_tamaño(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes <1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let ruta = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    // Leemos el contenido de la carpeta
    match fs::read_dir(ruta) {
        Ok(entries) => {
            println!("Contenido de: {}\n", ruta);
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = entry.metadata().unwrap();
                    let name = path.file_name().unwrap().to_string_lossy();
                    // Tamaño en bytes
                    let tamaño = metadata.len();
                    
                    // Si es carpeta o archivo
                    if metadata.is_dir() {
                        println!("📁 {}/", name);
                    } else {
                        let tamaño_formateado = formato_tamaño(tamaño);
                        println!("📄 {} ({})", name, tamaño_formateado);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error al leer la carpeta: {}", e);
        }
    }
}