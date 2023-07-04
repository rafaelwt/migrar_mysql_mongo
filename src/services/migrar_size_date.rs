use base64::{engine::general_purpose, Engine as _};
use futures::stream::StreamExt;
use indicatif::ProgressBar;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};
use mysql::prelude::*;
use std::thread;
use std::time::Duration;

use crate::configuration;

pub async fn migrar_size_date() -> Result<(), Box<dyn std::error::Error>> {
    // mysql connection
    let mysql_url = configuration::obtener_variable("mysql_url").unwrap();
    let mysql_pool = mysql::Pool::new(mysql_url.as_str())?;
    let mut mysql_conn = mysql_pool.get_conn()?;
    // mongo connection
    let mongodb_url = configuration::obtener_variable("mongodb_url").unwrap();
    let client_options = ClientOptions::parse(mongodb_url).await?;

    // Crear el cliente de MongoDB
    let client = Client::with_options(client_options)?;
    
    let database = client.database("pagoalpaso_prod");
    let mongo_collection = database.collection::<Document>("tbl_docDigitalizados");

    // barra de progreso
    let filter = doc! {};
    let count = mongo_collection.count_documents(filter, None).await?;
    let bar = ProgressBar::new(count as u64);

    // Obtener documentos de mongo
    let mut cursor = mongo_collection.find(None, None).await?;

    while let Some(result) = cursor.next().await {
        let document = match result {
            Ok(document) => document,
            Err(e) => return Err(e.into()), // An error occurred
        };

        if let Ok(base64_obj) = document.get_str("szBase64_obj") {
            let decoded = general_purpose::STANDARD.decode(base64_obj).unwrap();
            let size = decoded.len() as i32; // get the size of decoded data

            // Get date from MySQL table
            let l_doc_digitalizado_id = document.get_i32("lDocDigitalizado_id")?;
            let l_cobranza_id = document.get_i32("lCobranza_id")?;
            let i_servicio_id = document.get_i32("iServicio_id")?;
            let s_registro_dt = document.get_str("sRegistro_dt")?;
            if !s_registro_dt.trim().is_empty() {
                continue;
            }
            let fecha_pago: Option<String> = mysql_conn.exec_first("SELECT DATE_FORMAT(dtCobranza_dt, '%Y-%m-%d %H:%i:%s') as dtCobranza_dt FROM tbl_cobranzas WHERE lCobranza_id = ? AND iServicio_id = ?", (l_cobranza_id,i_servicio_id))?;
            // update size and date in mongo document
            if let Some(date) = fecha_pago {
                let filter = doc! {"lDocDigitalizado_id": l_doc_digitalizado_id};
                let update = doc! {"$set": {"lArchivo_sz": size, "sRegistro_dt": date}};
                mongo_collection.update_one(filter, update, None).await?;
            }
        }
        bar.inc(1);
        thread::sleep(Duration::from_millis(5));
    }
    bar.finish();
    Ok(())
}
