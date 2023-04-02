use crate::prelude::*;

#[quantity]
#[ref_unit(Euro, "€", NONE, "Reference unit of quantity `Currency`")]
#[unit(ThousandEuro, "k €", KILO, 1e3)]
#[unit(MillionEuro, "M €", MEGA, 1e6)]
#[unit(BillionEuro, "B €", GIGA, 1e9)]
#[unit(TrillionEuro, "T €", TERA, 1e12)]
pub struct Currency {}