use std::env;
use std::fs;
use std::path::Path;

fn search_in_file(pattern: &str, path: &Path, case_insensitive: bool) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,   // puede ser binario, permisos, etc. — lo ignoramos
    };

    for (i, line) in content.lines().enumerate() {
        let matched = if case_insensitive {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };

        if matched {
            println!("{}:{}   {}", path.display(), i + 1, line);
        }
    }
}

fn search_in_dir(pattern: &str, dir: &Path, case_insensitive: bool) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            println!("Error leyendo directorio {}: {}", dir.display(), e);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        if path.is_dir() {
            search_in_dir(pattern, &path, case_insensitive);
        } else {
            search_in_file(pattern, &path, case_insensitive);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Uso: rgrep <patrón> <archivo_o_directorio>");
        return;
    }

    let case_insensitive = args.contains(&"-i".to_string());
    // Filtramos los argumentos "reales" (sin el nombre del programa ni flags)
    let real_args: Vec<&String> = args.iter().skip(1).filter(|a| *a != "-i").collect();

    if real_args.len() < 2 {
        println!("Uso: rgrep [-i] <patrón> <archivo_o_directorio>");
        return;
    }

    let pattern = real_args[0];
    let target = Path::new(real_args[1]);

    if target.is_dir() {
        search_in_dir(pattern, target, case_insensitive);
    } else {
        search_in_file(pattern, target, case_insensitive);
    }
}