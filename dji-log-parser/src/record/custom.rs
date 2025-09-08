use binrw::binread;
use chrono::{DateTime, Datelike, Utc};
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use tsify_next::Tsify;

#[binread]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[br(little)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct Custom {
    #[br(temp)]
    _camera_shoot: u8,
    #[br(temp)]
    _video_shoot: u8,
    pub h_speed: f32,
    pub distance: f32,
    #[br(
        map = |x: i64| {
            // Try milliseconds first (original format)
            if let Some(dt) = DateTime::from_timestamp(x / 1000, (x % 1000 * 1000000) as u32) {
                if dt.year() > 2010 && dt.year() < 2100 {
                    return dt;
                }
            }
            // Try microseconds (possible newer format)
            if let Some(dt) = DateTime::from_timestamp(x / 1000000, (x % 1000000 * 1000) as u32) {
                if dt.year() > 2010 && dt.year() < 2100 {
                    return dt;
                }
            }
            // Try nanoseconds (possible newer format)
            if let Some(dt) = DateTime::from_timestamp(x / 1000000000, (x % 1000000000) as u32) {
                if dt.year() > 2010 && dt.year() < 2100 {
                    return dt;
                }
            }
            // Try as seconds (fallback)
            if let Some(dt) = DateTime::from_timestamp(x, 0) {
                if dt.year() > 2010 && dt.year() < 2100 {
                    return dt;
                }
            }
            DateTime::default()
        },
        // We ensure the year is between 2010 and 2100 to avoid invalid data
        assert(update_timestamp.year() > 2010 && update_timestamp.year() < 2100)
    )]
    #[cfg_attr(target_arch = "wasm32", tsify(type = "string"))]
    pub update_timestamp: DateTime<Utc>,
}