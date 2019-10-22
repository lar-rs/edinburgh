
// pub use serde_json::from_str;
use async_std::io::Result;
use async_std::prelude::*;
use async_std::fs;
use async_std::path::{
    Path,
    PathBuf
};
use futures_timer::Delay;





pub async fn label(path:Path) -> Result<String> {
    let label = fs::read_to_string(path.join("label")).await?;
    Ok(label)
}

pub async fn unit(path: Path)-> Result<String> {
    let unit = fs::read_to_string(&path.join("unit")).await?;
    Ok(unit)
}

pub async fn address(path: Path)-> Result<String> {
    let address = fs::read_to_string(&path.join("address")).await?;
    Ok(address)
}

// pub async fn change_label<T: AsRef<Path>>(path: T,label:&str) -> Result<()> {
//     let mut file = fs::File::create(path.as_path()).await?;
//     let state  = serde_json::to_vec(&State::new().turn_on()).unwrap();
//     file.write_all(state.as_slice()).await?;
//     file.sync_data().await?;
//     info!("UV-Lamp turn OFF");
//     Ok(())
// }

// pub async fn interval(path:PathBuf) -> Result<Delay>

