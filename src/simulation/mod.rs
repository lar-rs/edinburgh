
use async_std::prelude::*;
use async_std::stream;
use crate::temperatures::TemperatureSensor;


pub async fn temperatures() ->impl Stream<Item = Result<TemperatureSensor>> {
    let s = stream::repeat_with(|| async {
        TemperatureSensor{
            label:"Temperatur simulation",
            unit: "C",
            current: 32.0,
            high: None,
            critical: None,
        }
     });

pin_utils::pin_mut!(s);
    let mut s = stream::repeat(TemperatureSensor::random()).take(3);

}


pub async fn signal()