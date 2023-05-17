use std::fs::OpenOptions;
use std::io::prelude::*;
pub async fn save_to_file(id: i32) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("migrated_ids.txt")?;
    writeln!(file, "{}", id)?;
    Ok(())
}
pub async fn write_separation() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("migrated_ids.txt")?;
    writeln!(file, "------------------------")?;
    Ok(())
}