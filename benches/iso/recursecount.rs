use std::io::Read;
use std::io::Seek;
use std::path::Path;

use opticaldisc::iso::IsoFs;
use opticaldisc::iso::Metadata;

fn recursecount<H>(iso: &mut IsoFs<H>, meta: &Metadata) -> usize
where
    H: Seek + Read,
{
    if meta.is_dir() {
        meta.read_dir(iso)
            .unwrap()
            .into_iter()
            .fold(1, |acc, child| acc + recursecount(iso, &child))
    } else {
        1
    }
}

#[bench]
fn opticaldisc_file(b: &mut ::test::test::Bencher) {
    let path = Path::new("static/iso/alpine.level1.iso");
    let mut iso = IsoFs::from_path(path).unwrap();
    let root = iso.metadata("/").unwrap();
    assert_eq!(recursecount(&mut iso, &root), 125);
    b.iter(|| recursecount(&mut iso, &root));
}

#[bench]
fn opticaldisc_memory(b: &mut ::test::test::Bencher) {
    let data = include_bytes!("../../static/iso/alpine.level1.iso");
    let mut iso = IsoFs::from_buffer(&data[..]).unwrap();
    let root = iso.metadata("/").unwrap();
    assert_eq!(recursecount(&mut iso, &root), 125);
    b.iter(|| recursecount(&mut iso, &root));
}
