//! A platform agnostic Rust driver for Edinburgh gas sensor, based
//! on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//! [Homepage]: https://edinburghsensors.com/
//! [Edinburgh]: https://edinburghsensors.com/products/oem-co2-sensor/gascard-ng
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
//!
//!
//! ## The Device
//!
//! The Sensirion SGP30 is a low-power gas sensor for indoor air quality
//! applications with good long-term stability. It has an I²C interface with TVOC
//! (*Total Volatile Organic Compounds*) and CO₂ equivalent signals.
//!
//! - [Datasheet](https://www.sensirion.com/file/datasheet_sgp30)
//! - [Product Page](https://www.sensirion.com/sgp)
//!
//! ## Usage
//!
//! ### Instantiating
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate edinburgh;
//!
//! use hal::{Delay, I2cdev};
//! use edinburgh::Edinburgh;
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = 0x58;
//! let mut sgp = Edinburgh::new(dev, address, Delay);
//! # }
//! ```
//!
//! ### Fetching Device Information
//!
//! You can fetch the serial number of your sensor as well as the [feature
//! set](struct.FeatureSet.html):
//!
//! ```no_run
//! # extern crate linux_embedded_hal as hal;
//! # extern crate edinburgh;
//! # use hal::{Delay, I2cdev};
//! # use edinburgh::Edinburgh;
//! use edinburgh::FeatureSet;
//!
//! # fn main() {
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! # let mut sgp = Edinburgh::new(dev, 0x58, Delay);
//! let serial_number: [u8; 6] = sgp.serial().unwrap();
//! let feature_set: FeatureSet = sgp.get_feature_set().unwrap();
//! # }
//! ```
//!
//! ### Doing Measurements
//!
//! Before you do any measurements, you need to initialize the sensor.
//!
//! ```no_run
//! # extern crate linux_embedded_hal as hal;
//! # extern crate edinburgh;
//! # use hal::{Delay, I2cdev};
//! # use edinburgh::Edinburgh;
//! # fn main() {
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! # let mut sgp = Edinburgh::new(dev, 0x58, Delay);
//! sgp.init().unwrap();
//! # }
//! ```
//!
//! The SGP30 uses a dynamic baseline compensation algorithm and on-chip
//! calibration parameters to provide two complementary air quality signals.
//! Calling this method starts the air quality measurement. **After
//! initializing the measurement, the `measure()` method must be called in
//! regular intervals of 1 second** to ensure proper operation of the dynamic
//! baseline compensation algorithm. It is the responsibility of the user of
//! this driver to ensure that these periodic measurements are being done!
//!
//! ```no_run
//! # extern crate embedded_hal;
//! # extern crate linux_embedded_hal as hal;
//! # extern crate edinburgh;
//! # use hal::I2cdev;
//! # use edinburgh::Edinburgh;
//! use embedded_hal::blocking::delay::DelayMs;
//! use hal::Delay;
//! use edinburgh::Measurement;
//!
//! # fn main() {
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! # let mut sgp = Edinburgh::new(dev, 0x58, Delay);
//! # sgp.init().unwrap();
//! loop {
//!     let measurement: Measurement = sgp.measure().unwrap();
//!     println!("CO₂eq parts per million: {}", measurement.co2eq_ppm);
//!     println!("TVOC parts per billion: {}", measurement.tvoc_ppb);
//!     Delay.delay_ms(1000u16 - 12);
//! }
//! # }
//! ```
//!
//! *(Note: In the example we're using a delay of 988 ms because the
//! measurement takes up to 12 ms according to the datasheet. In reality, it
//! would be better to use a timer-based approach instead.)*
//!
//! For the first 15 s after initializing the air quality measurement, the
//! sensor is in an initialization phase during which it returns fixed
//! values of 400 ppm CO₂eq and 0 ppb TVOC. After 15 s (15 measurements)
//! the values should start to change.
//!
//! A new init command has to be sent after every power-up or soft reset.
//!
//! ### Restoring Baseline Values
//!
//! The SGP30 provides the possibility to read and write the values of the
//! baseline correction algorithm. This feature is used to save the baseline in
//! regular intervals on an external non-volatile memory and restore it after a
//! new power-up or soft reset of the sensor.
//!
//! The [`get_baseline()`](struct.Edinburgh.html#method.get_baseline) method
//! returns the baseline values for the two air quality signals. After a
//! power-up or soft reset, the baseline of the baseline correction algorithm
//! can be restored by calling [`init()`](struct.Edinburgh.html#method.init)
//! followed by [`set_baseline()`](struct.Edinburgh.html#method.set_baseline).
//!
//! ```no_run
//! # extern crate linux_embedded_hal as hal;
//! # extern crate edinburgh;
//! # use hal::{I2cdev, Delay};
//! # use edinburgh::Edinburgh;
//! use edinburgh::Baseline;
//!
//! # fn main() {
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! # let mut sgp = Edinburgh::new(dev, 0x58, Delay);
//! # sgp.init().unwrap();
//! let baseline: Baseline = sgp.get_baseline().unwrap();
//! // …
//! sgp.init().unwrap();
//! sgp.set_baseline(&baseline).unwrap();
//! # }
//! ```
//!
//! ### Humidity Compensation
//!
//! The SGP30 features an on-chip humidity compensation for the air quality
//! signals (CO₂eq and TVOC) and sensor raw signals (H2 and Ethanol). To use
//! the on-chip humidity compensation, an absolute humidity value from an
//! external humidity sensor is required.
//!
//! ```no_run
//! # extern crate linux_embedded_hal as hal;
//! # extern crate edinburgh;
//! # use hal::{I2cdev, Delay};
//! # use edinburgh::Edinburgh;
//! use edinburgh::Humidity;
//!
//! # fn main() {
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! # let mut sgp = Edinburgh::new(dev, 0x58, Delay);
//! // This value must be obtained from a separate humidity sensor
//! let humidity = Humidity::from_f32(23.42).unwrap();
//!
//! sgp.init().unwrap();
//! sgp.set_humidity(Some(&humidity)).unwrap();
//! # }
//! ```
//!
//! After setting a new humidity value, this value will be used by the
//! on-chip humidity compensation algorithm until a new humidity value is
//! set. Restarting the sensor (power-on or soft reset) or calling the
//! function with a `None` value sets the humidity value used for
//! compensation to its default value (11.57 g/m³) until a new humidity
//! value is sent.

#![deny(unsafe_code)]
// #![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

// extern crate byteorder;
use embedded_hal as hal;
// extern crate num_traits;

use byteorder::{BigEndian, ByteOrder};
use hal::blocking::delay::{DelayMs, DelayUs};
// use hal::blocking::serial::{Read, Write, WriteRead};
// use hal::serial;
use hal::serial::{Read,Write};
// pub use nb::Error;


mod types;

pub use types::{Measurement, RawSignals, Baseline,FeatureSet, ProductType};


const CRC8_POLYNOMIAL: u8 = 0x31;


/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// Serial communication error
    Serial(E),
    /// CRC checksum validation failed
    Crc,
    /// User tried to measure the air quality without starting the
    /// initialization phase.
    NotInitialized,
}

/// Serial commands sent to the sensor.
#[derive(Debug, Copy, Clone)]
enum Command {
    /// Run an on-chip self-test.
    SelfTest,
    /// Initialize air quality measurements.
    InitAirQuality,
    /// Get a current air quality measurement.
    MeasureAirQuality,
    /// Measure raw signals.
    MeasureRawSignals,
    // Return the baseline value.
    // GetBaseline,
    // Set the baseline value.
    // SetBaseline,
    /// Set the feature set.
    GetFeatureSet,
}

impl Command {
    fn as_bytes(self) -> [u8; 2] {
        match self {
            Command::SelfTest => [0x20, 0x32],
            Command::InitAirQuality => [0x20, 0x03],
            Command::MeasureAirQuality => [0x20, 0x08],
            Command::MeasureRawSignals => [0x20, 0x50],
            // Command::GetBaseline => [0x20, 0x15],
            // Command::SetBaseline => [0x20, 0x1E],
            Command::GetFeatureSet => [0x20, 0x2F],
        }
    }
}


/// Driver for the SGP30
#[derive(Debug, Default)]
pub struct Edinburgh<S, D> {
    /// The concrete I²C device implementation.
    s: S,
    ///  Buffer
    // read_buf: [u8; 32],
    /// The I²C device address.
    address: u8,
    /// The concrete Delay implementation.
    delay: D,
    /// Whether the air quality measurement was initialized.
    initialized: bool,
}

impl<S, D, E> Edinburgh<S, D>
where
    S: Read<u8, Error = E> + Write<u8, Error = E>,
    D: DelayUs<u16> + DelayMs<u16>,
{
    /// Create a new instance of the Edinburgh driver.
    pub fn new(s: S, address: u8, delay: D) -> Self {
        Edinburgh {
            s,
            address,
            delay,
            initialized: false,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> S{
        self.s
    }

    /// Write an I²C command to the sensor.
    fn send_command(&mut self, command: Command) -> nb::Result<(), Error<E>> {
        // self.s
            // .write(self.address, &command.as_bytes())
            // .map_err(Error::Serial)
        Ok(())
    }

   

    /// Iterate over the provided buffer and validate the CRC8 checksum.
    ///
    /// If the checksum is wrong, return `Error::Crc`.
    ///
    /// Note: This method will consider every third byte a checksum byte. If
    /// the buffer size is not a multiple of 3, then not all data will be
    /// validated.
    fn validate_crc(&self, buf: &[u8]) -> nb::Result<(), Error<E>> {
        // for chunk inj buf.chunks(3) {
            // if chunk.len() == 3
            // && crc8(&[chunk[0], chunk[1]]) != chunk[2] {
                // return Err(EdinburgError::Crc);
            // }
        // }
        Ok(())
    }

    /// Read data into the provided buffer and validate the CRC8 checksum.
    ///
    /// If the checksum is wrong, return `Error::Crc`.
    ///
    /// Note: This method will consider every third byte a checksum byte. If
    /// the buffer size is not a multiple of 3, then not all data will be
    /// validated.
    fn read_with_crc(&mut self, mut buf: &mut [u8]) -> nb::Result<(), Error<E>> {
        // self.s
            // .read(self.address, &mut buf)
            // .map_err(Error::I2c)?;
        self.validate_crc(buf)
    }


    /// Run an on-chip self-test. Return a boolean indicating whether the test succeeded.
    pub fn selftest(&mut self) -> nb::Result<bool, Error<E>> {
        // Start self test
        self.send_command(Command::SelfTest)?;

        // Max duration according to datasheet (Table 10)
        self.delay.delay_ms(220);

        // Read result
        let mut buf = [0; 3];
        self.read_with_crc(&mut buf)?;

        // Compare with self-test success pattern
        Ok(buf[0..2] == [0xd4, 0x00])
    }

    /// Initialize the air quality measurement.
    ///
    /// The Edinburgh uses a dynamic baseline compensation algorithm and on-chip
    /// calibration parameters to provide two complementary air quality
    /// signals.
    ///
    /// Calling this method starts the air quality measurement. After
    /// initializing the measurement, the `measure()` method must be called in
    /// regular intervals of 1 s to ensure proper operation of the dynamic
    /// baseline compensation algorithm. It is the responsibility of the user
    /// of this driver to ensure that these periodic measurements are being
    /// done.
    ///
    /// For the first 15 s after initializing the air quality measurement, the
    /// sensor is in an initialization phase during which it returns fixed
    /// values of 400 ppm CO₂eq and 0 ppb TVOC. After 15 s (15 measurements)
    /// the values should start to change.
    ///
    /// A new init command has to be sent after every power-up or soft reset.
    pub fn init(&mut self) -> nb::Result<(), Error<E>> {
        if self.initialized {
            // Already initialized
            return Ok(());
        }
        self.force_init()
    }

    /// Like [`init()`](struct.Edinburgh.html#method.init), but without checking
    /// whether the sensor is already initialized.
    ///
    /// This might be necessary after a sensor soft or hard reset.
    pub fn force_init(&mut self) -> nb::Result<(), Error<E>> {
        // Send command to sensor
        self.send_command(Command::InitAirQuality)?;

        // Max duration according to datasheet (Table 10)
        self.delay.delay_ms(10);

        self.initialized = true;
        Ok(())
    }

    /// Get measurement.
    ///
    /// Before calling this method, the air quality measurements must have been
    /// initialized using the [`init()`](struct.Edinburgh.html#method.init) method.
    /// Otherwise an [`Error::NotInitialized`](enum.Error.html#variant.NotInitialized)
    /// will be returned.
    ///
    /// Once the measurements have been initialized, the
    /// [`measure()`](struct.Edinburgh.html#method.measure) method must be called
    /// in regular intervals of 1 s to ensure proper operation of the dynamic
    /// baseline compensation algorithm. It is the responsibility of the user
    /// of this driver to ensure that these periodic measurements are being
    /// done.
    ///
    /// For the first 15 s after initializing the air quality measurement, the
    /// sensor is in an initialization phase during which it returns fixed
    /// values of 400 ppm CO₂eq and 0 ppb TVOC. After 15 s (15 measurements)
    /// the values should start to change.
    pub fn measure(&mut self) -> nb::Result<Measurement, Error<E>> {
        if !self.initialized {
            // Measurements weren't initialized
            return Err(nb::Error::Other(Error::NotInitialized))
        }
        // Send command to sensor
        self.send_command(Command::MeasureAirQuality)?;

        // Max duration according to datasheet (Table 10)
        self.delay.delay_ms(12);

        // Read result
        let mut buf = [0; 6];
        self.read_with_crc(&mut buf)?;
        let co2eq_ppm = (u16::from(buf[0]) << 8) | u16::from(buf[1]);
        let tvoc_ppb = (u16::from(buf[3]) << 8) | u16::from(buf[4]);
        let fsr = 0.0;
        Ok(Measurement {
            co2eq_ppm,
            tvoc_ppb,
            fsr,
        })
    }

    /// Return sensor raw signals.
    ///
    /// This command is intended for part verification and testing purposes. It
    /// returns the raw signals which are used as inputs for the on-chip
    /// calibration and baseline compensation algorithm. The command performs a
    /// measurement to which the sensor responds with the two signals for H2
    /// and Ethanol.
    pub fn measure_raw_signals(&mut self) -> nb::Result<RawSignals, Error<E>> {
        if !self.initialized {
            // Measurements weren't initialized
            return Err(nb::Error::Other(Error::NotInitialized));
        }

        // Send command to sensor
        self.send_command(Command::MeasureRawSignals)?;

        // Max duration according to datasheet (Table 10)
        self.delay.delay_ms(25);

        // Read result
        let mut buf = [0; 6];
        self.read_with_crc(&mut buf)?;
        let h2_signal = (u16::from(buf[0]) << 8) | u16::from(buf[1]);
        let ethanol_signal = (u16::from(buf[3]) << 8) | u16::from(buf[4]);

        Ok(RawSignals {
            h2: h2_signal,
            ethanol: ethanol_signal,
        })
    }

    /// Return the baseline values of the baseline correction algorithm.
    ///
    /// The Edinburgh provides the possibility to read and write the baseline
    /// values of the baseline correction algorithm. This feature is used to
    /// save the baseline in regular intervals on an external non-volatile
    /// memory and restore it after a new power-up or soft reset of the sensor.
    ///
    /// This function returns the baseline values for the two air quality
    /// signals. These two values should be stored on an external memory. After
    /// a power-up or soft reset, the baseline of the baseline correction
    /// algorithm can be restored by calling
    /// [`init()`](struct.Edinburgh.html#method.init) followed by
    /// [`set_baseline()`](struct.Edinburgh.html#method.set_baseline).
    pub fn get_baseline(&mut self) -> nb::Result<Baseline, Error<E>> {
        // Send command to sensor
        // self.send_command(Command::GetBaseline)?;

        // Max duration according to datasheet (Table 10)
        self.delay.delay_ms(10);

        // Read result
        // let mut buf = [0; 6];
        // self.read_with_crc(&mut buf)?;
        let co2eq_baseline = 0;//(u16::from(buf[0]) << 8) | u16::from(buf[1]);
        let tvoc_baseline = 0;//(u16::from(buf[3]) << 8) | u16::from(buf[4]);

        Ok(Baseline {
            co2eq: co2eq_baseline,
            tvoc: tvoc_baseline,
        })
    }

    /// Set the baseline values for the baseline correction algorithm.
    ///
    /// Before calling this method, the air quality measurements must have been
    /// initialized using the [`init()`](struct.Edinburgh.html#method.init) method.
    /// Otherwise an [`Error::NotInitialized`](enum.Error.html#variant.NotInitialized)
    /// will be returned.
    ///
    /// The Edinburgh provides the possibilty to read and write the baseline
    /// values of the baseline correction algorithm. This feature is used to
    /// save the baseline in regular intervals on an external non-volatile
    /// memory and restore it after a new power-up or soft reset of the sensor.
    ///
    /// This function sets the baseline values for the two air quality
    /// signals.
    pub fn set_baseline(&mut self, baseline: &Baseline) -> nb::Result<(), Error<E>> {
        if !self.initialized {
            // Measurements weren't initialized
            return Err(nb::Error::Other(Error::NotInitialized));
        }

        // Send command and data to sensor
        // Note that the order of the two parameters is inverted when writing
        // compared to when reading.
        let mut buf = [0; 4];
        BigEndian::write_u16(&mut buf[0..2], baseline.tvoc);
        BigEndian::write_u16(&mut buf[2..4], baseline.co2eq);
        // self.send_command_and_data(Command::SetBaseline, &buf)?;

        // Max duration according to datasheet (Table 10)
        self.delay.delay_ms(10);

        Ok(())
    }

    /// Get the feature set.
    ///
    /// The Edinburgh features a versioning system for the available set of
    /// measurement commands and on-chip algorithms. This so called feature set
    /// version number can be read out with this method.
    pub fn get_feature_set(&mut self) -> nb::Result<FeatureSet, Error<E>> {
        // Send command to sensor
        self.send_command(Command::GetFeatureSet)?;

        // Max duration according to datasheet (Table 10)
        self.delay.delay_ms(2);

        // Read result
        let mut buf = [0; 3];
        self.read_with_crc(&mut buf)?;

        Ok(FeatureSet::parse(buf[0], buf[1]))
    }
}

/// Calculate the CRC8 checksum.
///
/// Implementation based on the reference implementation by Sensirion.
fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xff;
    for byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) > 0 {
                crc = (crc << 1) ^ CRC8_POLYNOMIAL;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;

    use super::*;
    use self::hal::delay::MockNoop as DelayMock;
    use self::hal::i2c::{Mock as I2cMock, Transaction};

    /// Test the crc8 function against the test value provided in the
    /// datasheet (section 6.6).
    #[test]
    fn crc8_test_value() {
        assert_eq!(crc8(&[0xbe, 0xef]), 0x92);
    }

    /// Test the `validate_crc` function.
    #[test]
    fn validate_crc() {
        let mock = I2cMock::new(&[]);
        let sgp = Edinburgh::new(mock, 0x58, DelayMock);

        // Not enough data
        sgp.validate_crc(&[]).unwrap();
        sgp.validate_crc(&[0xbe]).unwrap();
        sgp.validate_crc(&[0xbe, 0xef]).unwrap();

        // Valid CRC
        sgp.validate_crc(&[0xbe, 0xef, 0x92]).unwrap();

        // Invalid CRC
        match sgp.validate_crc(&[0xbe, 0xef, 0x91]) {
            Err(Error::Crc) => {},
            Err(_) => panic!("Invalid error: Must be Crc"),
            Ok(_) => panic!("CRC check did not fail"),
        }

        // Valid CRC (8 bytes)
        sgp.validate_crc(&[0xbe, 0xef, 0x92, 0xbe, 0xef, 0x92, 0x00, 0x00]).unwrap();

        // Invalid CRC (8 bytes)
        match sgp.validate_crc(&[0xbe, 0xef, 0x91, 0xbe, 0xef, 0xff, 0x00, 0x00]) {
            Err(Error::Crc) => {},
            Err(_) => panic!("Invalid error: Must be Crc"),
            Ok(_) => panic!("CRC check did not fail"),
        }

        sgp.destroy().done();
    }

    /// Test the `read_with_crc` function.
    #[test]
    fn read_with_crc() {
        let mut buf = [0; 3];

        // Valid CRC
        let expectations = [
            Transaction::read(0x58, vec![0xBE, 0xEF, 0x92]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        sgp.read_with_crc(&mut buf).unwrap();
        assert_eq!(buf, [0xbe, 0xef, 0x92]);
        sgp.destroy().done();

        // Invalid CRC
        let expectations = [
            Transaction::read(0x58, vec![0xBE, 0xEF, 0x00]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        match sgp.read_with_crc(&mut buf) {
            Err(Error::Crc) => {},
            Err(_) => panic!("Invalid error: Must be Crc"),
            Ok(_) => panic!("CRC check did not fail"),
        }
        assert_eq!(buf, [0xbe, 0xef, 0x00]); // Buf was changed
        sgp.destroy().done();
    }

    /// Test the `serial` function
    #[test]
    fn serial() {
        let expectations = [
            Transaction::write(0x58, Command::GetSerial.as_bytes()[..].into()),
            Transaction::read(0x58, vec![0, 0, 129, 0, 100, 254, 204, 130, 135]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        let serial = sgp.serial().unwrap();
        assert_eq!(serial, [0, 0, 0, 100, 204, 130]);
        sgp.destroy().done();
    }

    /// Test the `selftest` function
    #[test]
    fn selftest_ok() {
        let expectations = [
            Transaction::write(0x58, Command::SelfTest.as_bytes()[..].into()),
            Transaction::read(0x58, vec![0xD4, 0x00, 0xC6]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        assert!(sgp.selftest().unwrap());
    }

    /// Test the `selftest` function
    #[test]
    fn selftest_fail() {
        let expectations = [
            Transaction::write(0x58, Command::SelfTest.as_bytes()[..].into()),
            Transaction::read(0x58, vec![0x12, 0x34, 0x37]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        assert!(!sgp.selftest().unwrap());
    }

    /// Test the `measure` function: Require initialization
    #[test]
    fn measure_initialization_required() {
        let mock = I2cMock::new(&[]);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        match sgp.measure() {
            Err(Error::NotInitialized) => {},
            Ok(_) => panic!("Error::NotInitialized not returned"),
            Err(_) => panic!("Wrong error returned"),
        }
        sgp.destroy().done();
    }

    /// Test the `measure` function: Calculation of return values
    #[test]
    fn measure_success() {
        let expectations = [
            Transaction::write(0x58, Command::InitAirQuality.as_bytes()[..].into()),
            Transaction::write(0x58, Command::MeasureAirQuality.as_bytes()[..].into()),
            Transaction::read(0x58, vec![0x12, 0x34, 0x37, 0xD4, 0x02, 0xA4]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        sgp.init().unwrap();
        let measurements = sgp.measure().unwrap();
        assert_eq!(measurements.co2eq_ppm, 4_660);
        assert_eq!(measurements.tvoc_ppb, 54_274);
        sgp.destroy().done();
    }

    /// Test the `get_baseline` function
    #[test]
    fn get_baseline() {
        let expectations = [
            Transaction::write(0x58, Command::InitAirQuality.as_bytes()[..].into()),
            Transaction::write(0x58, Command::GetBaseline.as_bytes()[..].into()),
            Transaction::read(0x58, vec![0x12, 0x34, 0x37, 0xD4, 0x02, 0xA4]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        sgp.init().unwrap();
        let baseline = sgp.get_baseline().unwrap();
        assert_eq!(baseline.co2eq, 4_660);
        assert_eq!(baseline.tvoc, 54_274);
        sgp.destroy().done();
    }

    /// Test the `set_baseline` function
    #[test]
    fn set_baseline() {
        let expectations = [
            Transaction::write(0x58, Command::InitAirQuality.as_bytes()[..].into()),
            Transaction::write(0x58, vec![
                /* command: */ 0x20, 0x1E,
                /* data + crc8: */ 0x56, 0x78, 0x7D, 0x12, 0x34, 0x37,
            ]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        sgp.init().unwrap();
        let baseline = Baseline {
            co2eq: 0x1234,
            tvoc: 0x5678,
        };
        sgp.set_baseline(&baseline).unwrap();
        sgp.destroy().done();
    }

    /// Test the `set_humidity` function
    #[test]
    fn set_humidity() {
        let expectations = [
            Transaction::write(0x58, Command::InitAirQuality.as_bytes()[..].into()),
            Transaction::write(0x58, vec![
                /* command: */ 0x20, 0x61,
                /* data + crc8: */ 0x0F, 0x80, 0x62,
            ]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        sgp.init().unwrap();
        let humidity = Humidity::from_f32(15.5).unwrap();
        sgp.set_humidity(Some(&humidity)).unwrap();
        sgp.destroy().done();
    }

    /// Test the `set_humidity` function with a None value
    #[test]
    fn set_humidity_none() {
        let expectations = [
            Transaction::write(0x58, Command::InitAirQuality.as_bytes()[..].into()),
            Transaction::write(0x58, vec![
                /* command: */ 0x20, 0x61,
                /* data + crc8: */ 0x00, 0x00, 0x81,
            ]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        sgp.init().unwrap();
        sgp.set_humidity(None).unwrap();
        sgp.destroy().done();
    }

    /// Test the `get_feature_set` function.
    #[test]
    fn get_feature_set() {
        let expectations = [
            Transaction::write(0x58, Command::InitAirQuality.as_bytes()[..].into()),
            Transaction::write(0x58, Command::GetFeatureSet.as_bytes()[..].into()),
            Transaction::read(0x58, vec![0x00, 0x42, 0xDE]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        sgp.init().unwrap();
        let feature_set = sgp.get_feature_set().unwrap();
        assert_eq!(feature_set.product_type, ProductType::Edinburgh);
        assert_eq!(feature_set.product_version, 0x42);
        sgp.destroy().done();
    }

    /// Test the `measure_raw_signals` function.
    #[test]
    fn measure_raw_signals() {
        let expectations = [
            Transaction::write(0x58, Command::InitAirQuality.as_bytes()[..].into()),
            Transaction::write(0x58, Command::MeasureRawSignals.as_bytes()[..].into()),
            Transaction::read(0x58, vec![0x12, 0x34, 0x37, 0x56, 0x78, 0x7D]),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sgp = Edinburgh::new(mock, 0x58, DelayMock);
        sgp.init().unwrap();
        let signals = sgp.measure_raw_signals().unwrap();
        assert_eq!(signals.h2, (0x12 << 8) + 0x34);
        assert_eq!(signals.ethanol, (0x56 << 8) + 0x78);
        sgp.destroy().done();
    }
}
