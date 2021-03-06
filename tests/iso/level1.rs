use std::io::Read;
use std::path::PathBuf;

use opticaldisc::iso::IsoFs;

lazy_static! {
    static ref path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("static")
        .join("iso")
        .join("alpine.level1.iso");
}

#[test]
fn test_is_dir() {
    let mut iso = IsoFs::from_path(path.as_path()).unwrap();
    assert!(iso.is_dir("ETC"));
    assert!(iso.is_dir("/ETC"));
    assert!(iso.is_dir("ETC/APK"));
    assert!(iso.is_dir("/ETC/APK"));
    assert!(!iso.is_dir("ETC/NODIR"));
    assert!(!iso.is_dir("/ETC/NODIR"));
    assert!(!iso.is_dir("ETC/APK/ARCHI"));
}

/// Checks the last directory record is parsed, i.e. the parser does stop
/// as intended before reaching the start of the data blocks.
#[test]
fn test_last_record() {
    let mut iso = IsoFs::from_path(path.as_path()).unwrap();
    assert!(iso.is_file("SBIN/APK"));
    assert!(iso.is_file("SBIN/LDCONFIG"));
    assert!(iso.is_file("SBIN/MKMNTDIR"));
    assert!(!iso.is_file("/"));
    assert!(!iso.is_file("/NOFILE"));
}

#[test]
fn test_metadata() {
    let mut iso = IsoFs::from_path(path.as_path()).unwrap();
    let arch = iso.metadata("/ETC/APK/ARCH").unwrap();
    assert!(arch.is_file());
    assert!(!arch.is_dir());
}
