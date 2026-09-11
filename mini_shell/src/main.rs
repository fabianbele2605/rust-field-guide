use std::io::{self, Write};
use std::process::Command;
use std::env;

fn parsear_comando(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn ejecutar_comando(comando: &str, argumentos: &[&str]) {
    match Command::new(comando)
        .args(argumentos)
        .spawn()
    {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            println!("Error: comando '{}' no encontrado ({})", comando, e);
        }
    }
}

fn mostrar_help() {
    println!("\n=== Mini Shell  - comandos Disponibles ===");
    println!("help              - Mostrar esta ayuda");
    println!("clear             - Limpiar pantalla");
    println!("cd <ruta>         - Cambiar directorio");
    println!("pwd               - Mostrar directorio actual");
    println!("exit              - Salir del sheel");
    println!("\nPuedes ejecutar cualquier comando del sistema:");
    println!("ls, cat, echo, mkdir, rm, etc.\n");
}

fn procesar_builtin(comando: &str, argumentos: &[&str]) -> bool {
    match comando {
        "exit" => {
            println!("Saliendo...");
            std::process::exit(0);
        }
        "cd" => {
            if argumentos.is_empty() {
                println!("cd: se requiere una ruta");
                return true;
            }
            match env::set_current_dir(argumentos[0]) {
                Ok(_) => true,
                Err(e) => {
                    println!("Error: no se pudo cambiar directorio ({})", e);
                    true
                }
            }
        }
        "help" => {
            mostrar_help();
            true
        }
        "clear" => {
            ejecutar_comando("clear", &[]);
            true
        }
        _ => false,
    }
}

fn main() {
    loop {
        print!("mysh> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let partes = parsear_comando(input.trim());
        
        if partes.is_empty() {
            continue;
        }
        
        let comando = partes[0];
        let argumentos = &partes[1..];
        
        if !procesar_builtin(comando, argumentos) {
            ejecutar_comando(comando, argumentos);
        }
    }
}
