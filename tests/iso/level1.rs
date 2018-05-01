use std::io::Read;

use opticaldisc::iso::IsoFs;

#[test]
fn test_iso_level1() {

    let mut path = ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("static");
    path.push("iso");
    path.push("alpine.level1.iso");

    let mut iso = IsoFs::from_path(path).unwrap();

    assert!(iso.is_dir("ETC"));
    assert!(iso.is_dir("ETC/APK"));
    assert!(iso.is_file("ETC/APK/ARCH"));
    assert!(!iso.is_dir("ETC/NODIR"));

    let arch = iso.metadata("/ETC/APK/ARCH").unwrap();
    assert!(arch.is_file());

    assert!(iso.is_dir("SBIN"));
    assert!(iso.is_file("SBIN/APK"));
    assert!(iso.is_file("SBIN/LDCONFIG"));
    assert!(iso.is_file("SBIN/MKMNTDIR"));
}
