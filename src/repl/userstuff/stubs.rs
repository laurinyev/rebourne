pub fn get_uid() -> u32 { 0 }
pub fn get_uname() -> String{ "root".to_string()}

#[cfg(feature = "safaos")]
pub fn get_hname() -> String{ "SafaOS".to_string()}

#[not(cfg(feature = "safaos"))]
pub fn get_hname() -> String{ "hostname".to_string()}
