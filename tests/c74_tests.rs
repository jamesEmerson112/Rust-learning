#[path = "../src/bin/c74_exercise.rs"]
#[allow(dead_code)]
mod c74_exercise;

use c74_exercise::intel::{Intel, decode_intel, encode_intel};
use c74_exercise::{crown_jewel, import_dump};

fn temp_vault() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn your_c71_codec_is_wired_in() {
    let db = temp_vault();
    let jewel = Intel { codename: "GHOSTKEY".to_string(), value: 64000 };
    encode_intel(&db, &jewel).unwrap();
    assert_eq!(
        decode_intel(&db, "GHOSTKEY").unwrap(),
        Some(jewel),
        "the capstone runs on YOUR c71 codec — finish c71 first"
    );
}

#[test]
fn imports_the_dump_and_totals_the_haul() {
    let path = "test_c74_vault_dump.csv";
    std::fs::write(path, "codename,value\nBLACKOUT,42000\nEXEC-DIRT,18500\nGHOSTKEY,64000\n").unwrap();
    let db = temp_vault();
    let haul = import_dump(&db, path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(haul, 124_500);
    assert_eq!(db.len(), 3);
}

#[test]
fn crown_jewel_is_the_most_valuable() {
    let db = temp_vault();
    for (codename, value) in [("BLACKOUT", 42000u32), ("GHOSTKEY", 64000), ("EXEC-DIRT", 18500)] {
        encode_intel(&db, &Intel { codename: codename.to_string(), value }).unwrap();
    }
    assert_eq!(
        crown_jewel(&db).unwrap(),
        Some(Intel { codename: "GHOSTKEY".to_string(), value: 64000 })
    );
}

#[test]
fn missing_dump_is_err() {
    let db = temp_vault();
    assert!(import_dump(&db, "nonexistent_dump.csv").is_err());
}
