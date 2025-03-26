use crate::Mime;
use fnv::FnvHashMap;

pub fn get_supported() -> Vec<Mime> {
    super::TYPES.to_vec()
}

/// Returns Vec of parent->child relations
pub fn get_subclasses() -> Vec<(Mime, Mime)> {
    vec![
        ("all/all", "all/allfiles"),
        ("all/all", "inode/directory"),
        ("all/allfiles", "application/octet-stream"),
        ("application/octet-stream", "text/plain"),
    ]
}

pub fn get_aliaslist() -> FnvHashMap<Mime, Mime> {
    FnvHashMap::default()
}
