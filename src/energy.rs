// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Energy`.

use crate::{force::Force, length::Length, prelude::*};

#[quantity(Force * Length)]
#[ref_unit(Joule, "J", NONE, "Reference unit of quantity `Energy`")]
#[unit(Newton_Meter, "Nm", NONE, 1, "N·m")]
#[unit(Watt_Second, "Ws", NONE, 1, "W·s")]
#[unit(Watt_Hour, "Wh", NONE, 3.6e3, "W·h")]
#[unit(Kilowatt_Hour, "kWh", KILO, 3.6e6, "kW·h")]
#[unit(Megawatt_Hour, "MWh", MEGA, 3.6e9, "MW·h")]
#[unit(Gigawatt_Hour, "GWh", GIGA, 3.6e12, "GW·h")]
/// Property that must be transferred to an object in order to perform work on
/// or to heat it.
///
/// Definition: Force·Length
///
/// Reference unit: Joule ('J' = 'N·m' = 'kg·m²/s²')
///
/// Predefined units:
///
/// | Symbol | Name                    | Definition        | Equivalent in 'J' |
/// |--------|-------------------------|-------------------|-------------------|
/// | Nm     | Newton Meter            | N·m               | 1                 |
/// | Ws     | Watt Second             | W·s               | 1                 |
/// | kWh    | Kilowatt Hour           | kW·h              | 3600000           |
pub struct Energy {}

impl Default for Energy {
    fn default() -> Self {
        0.0 * WATT_HOUR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_almost_eq, force::NEWTON, length::KILOMETER};

    #[test]
    fn test_force_mul_length() {
        let af = Amnt!(785.3);
        let f = af * NEWTON;
        let al: AmountT = Amnt!(38.4);
        let l = al * KILOMETER;
        let e = f * l;
        assert_almost_eq!(e.amount(), af * al * Amnt!(1000.));
        assert_eq!(e.unit(), JOULE);
    }
}
