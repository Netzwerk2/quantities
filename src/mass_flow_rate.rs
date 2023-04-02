use qty_macros::quantity;

use crate::duration::Duration;
use crate::mass::Mass;
use crate::prelude::*;

#[quantity(Mass / Duration)]
#[ref_unit(Kilogram_per_Second, "kg/s", KILO, "Reference unit of quantity `MassFlowRate`")]
#[unit(Tonne_per_Second, "t/s", MEGA, 1e3)]
#[unit(Tonne_per_Month, "t/month", MEGA, 2.592e6)]
#[unit(Kilotonne_per_Month, "kt/month", GIGA, 2.592e9)]
#[unit(Megatonne_per_Month, "Mt/month", TERA, 2.592e12)]
#[unit(Gigatonne_per_Month, "Gt/month", PETA, 2.592e15)]
pub struct MassFlowRate {}
