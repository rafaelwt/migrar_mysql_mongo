use dialoguer::{Confirm, Select};
use std::io::prelude::*;
mod configuration;
use tokio;
mod models;
mod utils;
mod services;
use crate::services::migrar::{migrar};
use crate::services::migrar_lote::{migrar_lote};
use crate::services::migrar_test::{migrar_test};
// use ctrlc;
#[tokio::main]
async fn main() {

    
    match configuration::validar_configuracion() {
        Ok(()) => {
            mostrar_menu().await;
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("Presiona cualquier tecla para salir...");
            let _ = std::io::stdout().flush();
            let _ = std::io::stdin().read(&mut [0u8]).unwrap();
        }
    }
}

async fn mostrar_menu() {
    loop {
        let menu_options = vec!["Migrar","Migrar por lote", "Prueba", "Salir"];
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
                    if let Err(err) = migrar().await {
                        eprintln!("Error al migrar los datos: {}", err);
                    } else {
                        println!("Migración completada exitosamente.");
                    }
                    println!("Presiona cualquier tecla para salir...");
                    let _ = std::io::stdout().flush();
                    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
                    break;
                }
            }
            1 => {
                let lote = configuration::obtener_variable("lote").unwrap();
                let mensaje = format!("¿Estás seguro de iniciar la migración? por lotes de : {} registros", lote); 
                let confirm = Confirm::new()
                    .with_prompt(mensaje)
                    .interact()
                    .unwrap();

                if confirm {
                    if let Err(err) = migrar_lote().await {
                        eprintln!("Error al migrar los datos: {}", err);
                    } else {
                        println!("Migración completada exitosamente.");
                    }
                    println!("Presiona cualquier tecla para salir...");
                    let _ = std::io::stdout().flush();
                    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
                    break;
                }
            },
            2 => {
                let limite_test = configuration::obtener_variable("limite_test").unwrap();
                let mensaje = format!("¿Estás seguro de iniciar la prueba de migración? Solo los primeros: {} registros", limite_test); 
                let confirm = Confirm::new()
                    .with_prompt(mensaje)
                    .interact()
                    .unwrap();

                if confirm {
                    if let Err(err) = migrar_test().await {
                        eprintln!("Error al migrar los datos: {}", err);
                    } else {
                        println!("Migración completada exitosamente.");
                    }
                    println!("Presiona cualquier tecla para salir...");
                    let _ = std::io::stdout().flush();
                    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
                    break;
                }
            }
            3 => {
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


