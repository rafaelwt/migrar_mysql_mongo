use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Documento {
    pub l_doc_digitalizado_id: i32,
    pub i_servicio_id: i32,
    pub l_cobranza_id: i64,
    pub s_doc_servicio_id: String,
    pub e_doc_digitalizado_fl: String,
    pub s_doc_digitalizado_nm: String,
    pub sz_base64_obj: String,
    pub e_estado_fl: String,
    pub e_migrado_fl: String,
}