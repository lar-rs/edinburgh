/// Configuration parameter edinburgh sensor.

const PORT_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud57600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone,
};


lazy_static::lazy_static! {

}



pub enum Model {
    Edin500,
}


pub struct Config {
    pub model:Model,
    pub min: f64,
    pub max: f64,
}


pub fn port_default()-> serial::PortSettings {
    PORT_SETTINGS.clone()
}



// pub fn workdir() -> PathBuf {
    // Path
// }


