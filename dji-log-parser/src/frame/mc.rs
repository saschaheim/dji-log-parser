use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use tsify_next::Tsify;

use crate::record::mc_param::FailSafeProtectionType;

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct FrameMC {
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub fail_safe_action: Option<FailSafeProtectionType>,
    pub mvo_func_enabled: bool,
    pub is_obstacle_avoidance_enabled: bool,
    pub user_avoid_enabled: bool,
}
