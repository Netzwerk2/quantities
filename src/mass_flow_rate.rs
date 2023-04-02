use qty_macros::quantity;

use crate::duration::Duration;
use crate::mass::Mass;
use crate::prelude::*;

#[quantity(Mass / Duration)]
#[ref_unit(Kilogram_per_Second, "kg/s", KILO, "Reference unit of quantity `MassFlowRate`")]
#[unit(Tonne_per_Second, "t/s", MEGA, 1e-3)]
pub struct MassFlowRate {}
