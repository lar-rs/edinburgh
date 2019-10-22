

use super::Sensor;








/// Check ndir format
pub async fn check(sensor: &Sensor) -> bool {
    let path = sensor.path.join("ndir");
    let test = if path.exists() {
        true
    } else {
        false
    };
    test
}
