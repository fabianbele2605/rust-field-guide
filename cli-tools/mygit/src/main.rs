use std::env;
use std::fs;
use sha2::{Sha256, Digest};

fn cmd_init() {
    fs::create_dir_all(".mygit/objects").unwrap();
    fs::write(".mygit/HEAD", "").unwrap();  // vacío = sin commits todavia
    println!("Repositorio inicializado en .mygit/");
}

fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result)  // convierte los bytes del hash a hexadecimal
}

fn cmd_add(filename: &str) {
    let content = match fs::read(filename) {
        Ok(c) => c,
        Err(e) => {
            println!("Error leyendo {}: {}", filename, e);
            return;
        }
    };

    let hash = hash_content(&content);
    let object_path = format!(".mygit/objects/{}", hash);
    fs::write(&object_path, &content).unwrap();

    // Agregar al indice (formato simple: "archivo hash" por linea)
    let index_line = format!("{} {}\n", filename, hash);
    let mut index_content = fs::read_to_string(".mygit/index").unwrap_or_default();

    // Si el archivo ya estaba en el indice, reemplaza esa linea
    let mut lines: Vec<String> = index_content
        .lines()
        .filter(|l| !l.starts_with(&format!("{} ", filename)))
        .map(|l| l.to_string())
        .collect();
    lines.push(index_line.trim().to_string());
    index_content = lines.join("\n") + "\n";

    fs::write(".mygit/index", index_content).unwrap();

    println!("Agregado {} (hash: {})", filename, &hash[..8]);
}

fn cmd_commit(message: &str) {
    let index_content = fs::read_to_string(".mygit/index").unwrap_or_default();
    if index_content.trim().is_empty() {
        println!("Nada para commitear (usa 'mygit add' primero)");
        return;
    }

    let parent = fs::read_to_string(".mygit/HEAD").unwrap_or_default();
    let parent = parent.trim();

    let commit_content = format!(
        "parent: {}\nmessage: {}\nindex:\n{}",
        if parent.is_empty() { "none" } else { parent },
        message,
        index_content
    );

    let hash = hash_content(commit_content.as_bytes());
    let object_path = format!(".mygit/objects/{}", hash);
    fs::write(&object_path, &commit_content).unwrap();
    fs::write(".mygit/HEAD", &hash).unwrap();

    println!("Commit creado: {} - \"{}\"", &hash[..8], message);
}

fn cmd_log() {
    let current = fs::read_to_string(".mygit/HEAD").unwrap_or_default();
    let mut current = current.trim().to_string();

    if current.is_empty() {
        println!("Sin commits todavía");
        return;
    }

    loop {
        let object_path = format!(".mygit/objects/{}", current);
        let content = match fs::read_to_string(&object_path) {
            Ok(c) => c,
            Err(_) => break,
        };

        // Extraer "parent:..." y "message: ..." de las primeras 2 lineas
        let mut parent = String::new();
        let mut message = String::new();
        for line in content.lines() {
            if let Some(p) = line.strip_prefix("parent: ") {
                parent = p.to_string();
            } else if let Some(m) = line.strip_prefix("message: ") {
                message = m.to_string();
            }
        }

        println!("commit {}", &current[..8]);
        println!("      {}\n", message);

        if parent == "none" || parent.is_empty() {
            break;
        }
        current = parent;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: mygit <init|add|commit|log> [argumentos]");
        return;
    }

    match args[1].as_str() {
        "init" => cmd_init(),
        "add" => {
            if args.len() < 3 {
                println!("Uso: mygit add <archivo>");
                return;
            }
            cmd_add(&args[2]);
        }
        "commit" => {
            if args.len() < 3 {
                println!("Uso: mygit commit <mensaje>");
                return;
            }
            cmd_commit(&args[2]);
        }
        "log" => cmd_log(),
        _ => println!("Comando desconocido: {}", args[1]),
    }
}