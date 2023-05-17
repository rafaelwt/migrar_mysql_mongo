// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;
// pub fn bloquear_control_c () {
//     // Evitar interrupir con control + c
//     let running = Arc::new(AtomicBool::new(true));
//     let r = running.clone();

//     ctrlc::set_handler(move || {
//         r.store(false, Ordering::SeqCst);
//     }).expect("Error setting Ctrl-C handler");
//     // ========================================
//     // Iniciar migración
// }