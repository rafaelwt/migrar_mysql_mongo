// use base64::{engine::general_purpose, Engine as _};
// use mongodb::bson;
// use mongodb::bson::Binary;
use mysql::params;
use mysql::prelude::*;
use indicatif::ProgressBar;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::thread;
use std::time::Duration;

use crate::configuration;
use crate::models;
use crate::utils::write_file::{save_to_file, write_separation};

pub async fn migrar() -> Result<(), Box<dyn std::error::Error>> {
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

    let count: i64 = conn
        .exec_first(
            "SELECT COUNT(*) from tbl_docDigitalizados WHERE eMigrado_fl = 'N'",
            (),
        )?
        .unwrap();
    let bar = ProgressBar::new(count as u64);

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
            models::documento::Documento {
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
    let db: Database = client.database("pagoalpaso_prod");
    let collection = db.collection("tbl_docDigitalizados");
    // let collection_byte = db.collection("tbl_docDigitalizados_byte");

    for doc in query_result {
        // let decoded_bytes = base64::decode("YmFzZTY0IGRlY29kZQ==").unwrap(); // deprecated
        // let decoded_bytes = general_purpose::STANDARD
        //     .decode(&doc.sz_base64_obj)
        //     .unwrap();
        let document = doc! {
            "lDocDigitalizado_id": &doc.l_doc_digitalizado_id,
            "iServicio_id": &doc.i_servicio_id,
            "lCobranza_id": &doc.l_cobranza_id,
            "sDocServicio_id": &doc.s_doc_servicio_id,
            "eDocDigitalizado_fl": &doc.e_doc_digitalizado_fl,
            "sDocDigitalizado_nm": &doc.s_doc_digitalizado_nm,
            "szBase64_obj": &doc.sz_base64_obj,
            "eEstado_fl": &doc.e_estado_fl,
            "eMigrado_fl": &doc.e_migrado_fl,
        };

        // let document_byte = doc! {
        //     "lDocDigitalizado_id": &doc.l_doc_digitalizado_id,
        //     "iServicio_id": &doc.i_servicio_id,
        //     "lCobranza_id": &doc.l_cobranza_id,
        //     "sDocServicio_id": &doc.s_doc_servicio_id,
        //     "eDocDigitalizado_fl": &doc.e_doc_digitalizado_fl,
        //     "sDocDigitalizado_nm": &doc.s_doc_digitalizado_nm,
        //     "szBase64_obj": Binary { subtype: bson::spec::BinarySubtype::Generic, bytes: decoded_bytes },
        //     "eEstado_fl": &doc.e_estado_fl,
        //     "eMigrado_fl": &doc.e_migrado_fl,
        // };
        collection.insert_one(document, None).await?;
        // collection_byte.insert_one(document_byte, None).await?;
        conn.exec_drop(
            r"UPDATE tbl_docDigitalizados SET eMigrado_fl = 'S' WHERE lDocDigitalizado_id = :id",
            params! {
                "id" => doc.l_doc_digitalizado_id
            },
        )?;
        save_to_file(doc.l_doc_digitalizado_id).await.unwrap();
        bar.inc(1);
        thread::sleep(Duration::from_millis(5));
    }
    if count > 0 {
        write_separation().await.unwrap();
    }

    bar.finish();
    Ok(())
}
