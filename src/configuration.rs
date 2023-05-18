use std::path::Path;
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use std::io;

pub fn validar_configuracion() -> Result<(), io::Error> {
    if !Path::new(".env").exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "El archivo .env no existe",
        ));
    }

    dotenv().ok();

    match env::var("mysql_url") {
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "La variable mysql_url no está definida en .env",
            ))
        }
        _ => (),
    }

    match env::var("mongodb_url") {
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "La variable mongodb_url  no está definida en .env",
            ))
        }
        _ => (),
    }
    match env::var("lote") {
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "La variable lote  no está definida en .env (debe ser un numero entero))",
            ))
        }
        _ => (),
    }

    Ok(())
}
pub fn obtener_variable(nombre: &str) -> Result<String, io::Error> {
    dotenv().ok();

    match env::var(nombre) {
        Ok(val) => Ok(val),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("La variable {} no está definida en .env", nombre),
        )),
    }
}
