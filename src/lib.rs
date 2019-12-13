//! ## The Device
//! A platform agnostic Rust driver for Edinburgh gas sensor, based
//! on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//! * [Homepage]:(https://edinburghsensors.com/)
//! * [Data]: https://edinburghsensors.com/products/oem-co2-sensor/gascard-ng
//! KEY FEATURES
//! On-board barometric pressure correction in the range 800mbar to 1150mbar.
//! Extensive temperature compensation.
//! Minimum operating voltage 7V and wide operating voltage range (7V to 30V).
//! True RS232 communications for control and data logging. Optional on-board LAN support.
//! # References
//! # GAS MEASUREMENT RANGE
//! Model	    CO2	CH4	CO
//! GasCard NG	-	0-5%	0-10%
//! GasCard NG	-	0-10%	0-30%
//! GasCard NG	0-2000 ppm	0-30%	0-100%
//! GasCard NG	0-3000 ppm	0-100%	-
//! CardCard NG	0-5000 ppm	-	-
//! GasCard NG	0-1%	-	-
//! GasCard NG	0-3%	-	-
//! GasCard NG	0-5%	-	-
//! GasCard NG	0-10%	-	-
//! GasCard NG	0-30%	-	-
//! GasCard NG	0-100%	-	-
//! Biogas	100%	100%	-
//! Accuracy	±2% of range ±<0.015% of range per mbar
//! Zero stability	±2% of range (over 12 months)
//! Response time	T90 = 10 seconds or programmable RC
//! Operating temperature	0-45ºC
//! Power requirements	24 V DC (7V-30V)
//! Warm-up time	1 minute (initial) 30 minutes (full specification)
//! Humidity	Measurements are unaffected by 0-95% relative humidity, non condensing
//! Output	Linear 4-20 mA, 0-20 mA (bit switch selectable) maximum load dependant on supply voltage
//! Please Note	Equipment is configured for one gas type at a time.





pub mod config;
pub mod banner;
pub mod cli;
pub mod driver;
pub mod edinburg;
