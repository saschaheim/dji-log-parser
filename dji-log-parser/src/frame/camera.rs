use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use tsify_next::Tsify;

use crate::record::camera::SDCardState;

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct FrameCamera {
    /// Indicates that one or more photos were captured in this frame.
    pub is_photo: bool,
    /// Number of photos captured in this frame.
    pub photo_count: u32,
    /// Indicates if the camera is in video mode
    pub is_video: bool,
    /// Indicates if an SD card is inserted
    pub sd_card_is_inserted: bool,
    /// Current state of the SD card
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub sd_card_state: Option<SDCardState>,
}
