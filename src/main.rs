use dialoguer::{Confirm, Select};
use std::fs::File;
use std::io::{self, prelude::*};
mod configuration;
fn main() {
    // Validar configuración
    // let settings = match validacion::validar_configuracion() {
    //     Ok(settings) => settings,
    //     Err(err) => {
    //         println!("Error al cargar el archivo de configuración: {}", err);
    //         return;
    //     }
    // };
    match configuration::validar_configuracion() {
        Ok(()) =>{
            mostrar_menu();
        },
        Err(e) =>  {
            println!("Error: {}", e);
            println!("Presiona cualquier tecla para salir...");
            let _ = std::io::stdout().flush();
            let _ = std::io::stdin().read(&mut [0u8]).unwrap();
        },
    }
   
}

fn mostrar_menu() {
    loop {
        let menu_options = vec!["Migrar", "Salir"];
        println!("==============  Sistema de migración de datos ===================");
        let selection = Select::new()
            .with_prompt("Menú")
            .items(&menu_options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let confirm = Confirm::new()
                    .with_prompt("¿Estás seguro de iniciar la migración?")
                    .interact()
                    .unwrap();

                if confirm {
                    migrar();
                    println!("Presiona cualquier tecla para salir...");
                    let _ = std::io::stdout().flush();
                    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
                    break;
                }
            }
            1 => {
                println!("Operación cancelada...");
                break;
            }
            _ => {
                println!("Opción inválida. Por favor, selecciona una opción válida.");
                continue;
            }
        }
    }
}
fn migrar() {
    let mysql_url = configuration::obtener_variable("mysql_url").unwrap();
    println!("El valor de mysql_url es: {}", mysql_url);
    let mongodb_url = configuration::obtener_variable("mongodb_url").unwrap();
    println!("El valor de mongodb_url es: {}", mongodb_url);
    println!("Iniciando migración...");
    // Lógica de migración aquí
    println!("Migración completada.");
}
fn guardar_ids_migrados(ids: &[i32]) -> io::Result<()> {
    let mut file = File::create("ids_migrados.txt")?;
    for id in ids {
        writeln!(file, "{}", id)?;
    }
    Ok(())
}
