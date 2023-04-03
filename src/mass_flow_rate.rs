use crate::duration::Duration;
use crate::mass::Mass;
use crate::prelude::*;

#[quantity(Mass / Duration)]
#[ref_unit(Kilogram_per_Second, "kg/s", KILO, "Reference unit of quantity `MassFlowRate`")]
#[unit(Tonne_per_Second, "t/s", MEGA, 1e3)]
#[unit(Tonne_per_Month, "t/month", MEGA, 0.3858024691e-3)]
#[unit(Kilotonne_per_Month, "kt/month", GIGA, 0.3858024691)]
#[unit(Megatonne_per_Month, "Mt/month", TERA, 0.3858024691e3)]
#[unit(Gigatonne_per_Month, "Gt/month", PETA, 0.3858024691e6)]
pub struct MassFlowRate {}

impl Default for MassFlowRate {
    fn default() -> Self {
        0.0 * KILOGRAM_PER_SECOND
    }
}
