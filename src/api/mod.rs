pub mod books;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = books::register_server_functions();
}