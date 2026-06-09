use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use tsify_next::Tsify;

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct FrameBattery {
    /// Battery index in multi-battery systems
    pub index: u8,
    /// Battery charge level in percentage
    pub charge_level: u8,
    /// Battery voltage
    pub voltage: f32,
    /// Battery current
    pub current: f32,
    /// Designed battery capacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design_capacity: Option<u32>,
    /// Current battery capacity
    pub current_capacity: u32,
    /// Full battery capacity
    pub full_capacity: u32,
    /// Number of battery cells
    pub cell_num: u8,
    /// Indicates if cell voltage is derived from global voltage
    pub is_cell_voltage_estimated: bool,
    /// Cell voltages
    pub cell_voltages: Vec<f32>,
    /// Deviation in cell voltages
    pub cell_voltage_deviation: Option<f32>,
    /// Maximum deviation in cell voltages
    pub max_cell_voltage_deviation: Option<f32>,
    /// Battery temperature
    pub temperature: f32,
    /// Minimum battery temperature
    pub min_temperature: f32,
    /// Maximum battery temperature
    pub max_temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_discharges: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime_remaining: Option<u8>,
}
