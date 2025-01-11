use crate::app::pocs::pocs::{PocConfig, PocTemplate};

#[derive(Debug)]
pub struct Cve20177921 {
    pub fields: PocTemplate,
}

// impl Cve20177921 {
//     pub fn new(config: PocConfig) -> Self {
//         let name = std::path::Path::new(file!())
//             .file_name()
//             .unwrap_or_default()
//             .to_string_lossy()
//             .split(".")
//             .next()
//             .unwrap_or_default().to_string();
//
//         let fields = PocTemplate{}
//
//     }
// }