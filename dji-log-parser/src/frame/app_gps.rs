use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use tsify_next::Tsify;

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct FrameAppGPS {
    /// App-provided latitude in degrees.
    pub latitude: f64,
    /// App-provided longitude in degrees.
    pub longitude: f64,
}
