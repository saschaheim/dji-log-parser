use binrw::binread;
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use tsify_next::Tsify;

use crate::utils::sanitize_fixed_width_string;

#[binread]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[br(little, import { length: u16 })]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct AppSeriousWarn {
    #[br(count=length, map = |s: Vec<u8>| sanitize_fixed_width_string(&s))]
    pub message: String,
}
