use dioxus::prelude::{asset, Asset};
use dioxus_use_js::use_js;

// Load d3 from CDN
// NOTE: Path is relative to d3_wrapper crate directory
use_js!("assets/d3_core.js"::{loadD3, isD3Loaded});

/// Initialize d3.js by loading from CDN
pub async fn init_d3() -> Result<(), dioxus_use_js::JsError> {
    let _loaded: bool = loadD3().await?;
    Ok(())
}

/// Check if d3.js is already loaded
pub async fn is_d3_loaded() -> Result<bool, dioxus_use_js::JsError> {
    isD3Loaded().await
}
