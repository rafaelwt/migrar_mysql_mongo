use indicatif::ProgressBar;
use mongodb::{bson::{doc , Document}, options::ClientOptions, Client};
use mysql::prelude::*;
use std::thread;
use std::time::Duration;

use crate::configuration;
use crate::models;
#[allow(dead_code)]
pub async fn migrar_date() -> Result<(), Box<dyn std::error::Error>> {
    let mysql_url = configuration::obtener_variable("mysql_url").unwrap();
    println!("El valor de mysql_url es: {}", mysql_url);
    let mysql_pool = mysql::Pool::new(mysql_url.as_str())?;
    let mut conn = mysql_pool.get_conn()?;

    let query_result = conn.query_map(
        "select doc.lDocDigitalizado_id, cob.dtCobranza_dt from tbl_docDigitalizados as doc, tbl_cobranzas as cob where doc.iServicio_id = cob.iServicio_id and doc.lCobranza_id = cob.lCobranza_id",
        |(
            l_doc_digitalizado_id,
            dt_cobranza_dt,
        )| {
            models::documento::Cobranza {
                l_doc_digitalizado_id: l_doc_digitalizado_id,
                dt_cobranza_dt: dt_cobranza_dt,
            }
        },
    )?;
    update_mongo_docs(query_result).await?;
    Ok(())
   
}
#[allow(dead_code)]
async fn update_mongo_docs(cobranzas: Vec<models::documento::Cobranza>) -> Result<(), Box<dyn std::error::Error>> {
    let count = cobranzas.len();
    // Progress bar
    let bar = ProgressBar::new(count as u64);
    // Configuración de conexión a MongoDB
    // Configurar la conexión
    
    let mongodb_url = configuration::obtener_variable("mongodb_url").unwrap();
    let client_options = ClientOptions::parse(mongodb_url).await?;

    // Crear el cliente de MongoDB
    let client = Client::with_options(client_options)?;
    
    let database = client.database("pagoalpaso_prod");
    let collection = database.collection::<Document>("tbl_docDigitalizados");

    for cobranza in cobranzas {
        let filter = doc! {"lDocDigitalizado_id": cobranza.l_doc_digitalizado_id};
        let update = doc! {"$set": {"sRegistro_dt": cobranza.dt_cobranza_dt}};
        let _ = collection.update_one(filter, update, None).await?;
        bar.inc(1);
        thread::sleep(Duration::from_millis(5));
    }

    bar.finish();
    Ok(())
}
