use dialoguer::{Confirm, Select};
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, prelude::*};
mod configuration;
use mongodb::{
    options::{ClientOptions},
    Client,
    Database,
    bson::{doc},
};
use tokio;
#[tokio::main]
async fn main() {
    // Validar configuración
    // let settings = match validacion::validar_configuracion() {
    //     Ok(settings) => settings,
    //     Err(err) => {
    //         println!("Error al cargar el archivo de configuración: {}", err);
    //         return;
    //     }
    // };
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
// fn migrar() {
//     let mysql_url = configuration::obtener_variable("mysql_url").unwrap();
//     println!("El valor de mysql_url es: {}", mysql_url);
//     let mongodb_url = configuration::obtener_variable("mongodb_url").unwrap();
//     println!("El valor de mongodb_url es: {}", mongodb_url);
//     println!("Iniciando migración...");
//     // Lógica de migración aquí
//     println!("Migración completada.");
// }
// fn guardar_ids_migrados(ids: &[i32]) -> io::Result<()> {
//     let mut file = File::create("ids_migrados.txt")?;
//     for id in ids {
//         writeln!(file, "{}", id)?;
//     }
//     Ok(())
// }
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Documento {
    l_doc_digitalizado_id: i32,
    i_servicio_id: i32,
    l_cobranza_id: i64,
    s_doc_servicio_id: String,
    e_doc_digitalizado_fl: String,
    s_doc_digitalizado_nm: String,
    sz_base64_obj: String,
    e_estado_fl: String,
    e_migrado_fl: String,
}

async fn migrar() -> Result<(), Box<dyn std::error::Error>> {
    let mysql_url = configuration::obtener_variable("mysql_url").unwrap();
    let mongodb_url = configuration::obtener_variable("mongodb_url").unwrap();
    println!("El valor de mysql_url es: {}", mysql_url);
    println!("El valor de mongodb_url es: {}", mongodb_url);
    let mysql_pool = mysql::Pool::new(mysql_url.as_str())?;
    let mut conn = mysql_pool.get_conn()?;
    // Configuración de conexión a MongoDB
    // Configurar la conexión
    let client_options = ClientOptions::parse(mongodb_url).await?;

    // Crear el cliente de MongoDB
    let client = Client::with_options(client_options)?;

    let query_result = conn.query_map(
        "SELECT lDocDigitalizado_id, iServicio_id, lCobranza_id, sDocServicio_id, eDocDigitalizado_fl, sDocDigitalizado_nm, szBase64_obj, eEstado_fl, eMigrado_fl FROM tbl_docDigitalizados WHERE eMigrado_fl = 'N'",
        |(
            l_doc_digitalizado_id,
            i_servicio_id,
            l_cobranza_id,
            s_doc_servicio_id,
            e_doc_digitalizado_fl,
            s_doc_digitalizado_nm,
            sz_base64_obj,
            e_estado_fl,
            e_migrado_fl,
        )| {
            Documento {
                l_doc_digitalizado_id: l_doc_digitalizado_id,
                i_servicio_id: i_servicio_id,
                l_cobranza_id: l_cobranza_id,
                s_doc_servicio_id: s_doc_servicio_id,
                e_doc_digitalizado_fl: e_doc_digitalizado_fl,
                s_doc_digitalizado_nm: s_doc_digitalizado_nm,
                sz_base64_obj: sz_base64_obj,
                e_estado_fl: e_estado_fl,
                e_migrado_fl: e_migrado_fl,
            }
        },
    )?;
    let db: Database = client.database("pagoalpaso");
    let collection = db.collection("tbl_docDigitalizados");

    for doc in query_result {
        let document = doc! {
            "lDocDigitalizado_id": doc.l_doc_digitalizado_id,
            "iServicio_id": doc.i_servicio_id,
            "lCobranza_id": doc.l_cobranza_id,
            "sDocServicio_id": doc.s_doc_servicio_id,
            "eDocDigitalizado_fl": doc.e_doc_digitalizado_fl,
            "sDocDigitalizado_nm": doc.s_doc_digitalizado_nm,
            "szBase64_obj": doc.sz_base64_obj,
            "eEstado_fl": doc.e_estado_fl,
            "eMigrado_fl": doc.e_migrado_fl,
        };
        collection.insert_one(document, None).await?;
    }


    Ok(())
}
