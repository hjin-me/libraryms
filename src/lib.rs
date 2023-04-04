use cfg_if::cfg_if;
use leptos::*;
pub mod api;
#[cfg(feature = "ssr")]
pub mod backend;
pub mod components;
pub mod entity;
pub mod error_template;
pub mod errors;
#[cfg(feature = "ssr")]
pub mod fallback;

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use components::home::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            // _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(|cx| {
                view! { cx,  <BlogApp/> }
            });
        }
    }
}
