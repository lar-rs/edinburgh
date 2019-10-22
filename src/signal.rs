
pub use serde_json::from_str;
use async_std::prelude::*;
use async_std::path::{
    Path,
    PathBuf
};
use async_std::fs;
use async_std::io;

use futures_timer::Delay;

use std::str::FromStr;




async fn current<T: AsRef<Path>>(path: T) -> Result<f32> {
    let path = path.join("current");
    let current = f32::from_str(fs::read_to_string(path).await?)?;
    Ok(current)

}


// async fn average

// async fn connect<T: AsRef<Path>>(path: T) -> impl Stream<Item = Result<f32>> {
    //    let mut stream = UnixStream::connect(path.join("signal")).await?;

//     let stream = TcpStream::connect(addr).await?;
//     let (reader, mut writer) = (&stream, &stream);
//     let reader = BufReader::new(reader);
//     let mut lines_from_server = futures::StreamExt::fuse(reader.lines());

//     let stdin = BufReader::new(stdin());
//     let mut lines_from_stdin = futures::StreamExt::fuse(stdin.lines());
//     loop {
//         select! {
//             line = lines_from_server.next().fuse() => match line {
//                 Some(line) => {
//                     let line = line?;
//                     println!("{}", line);
//                 },
//                 None => break,
//             },
//             line = lines_from_stdin.next().fuse() => match line {
//                 Some(line) => {
//                     let line = line?;
//                     writer.write_all(line.as_bytes()).await?;
//                     writer.write_all(b"\n").await?;
//                 }
//                 None => break,
//             }
//         }
//     }
//     Ok(())
// }
