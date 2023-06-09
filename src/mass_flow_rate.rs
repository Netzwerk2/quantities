use crate::duration::Duration;
use crate::mass::Mass;
use crate::prelude::*;

#[quantity(Mass / Duration)]
#[unit(Kilogram_per_Second, "kg/s", MILLI, 2629.8)]
#[unit(Tonne_per_Second, "t/s", NONE, 2629800)]
#[ref_unit(Tonne_per_Month, "t/month", NONE, "Reference unit of quantity `MassFlowRate`")]
#[unit(Kilotonne_per_Month, "kt/month", KILO, 1e3)]
#[unit(Megatonne_per_Month, "Mt/month", MEGA, 1e6)]
#[unit(Gigatonne_per_Month, "Gt/month", GIGA, 1e9)]
pub struct MassFlowRate {}

impl Default for MassFlowRate {
    fn default() -> Self {
        0.0 * TONNE_PER_MONTH
    }
}
