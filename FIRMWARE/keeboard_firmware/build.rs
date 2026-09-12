use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    println!("cargo:rerun-if-changed=keyboard.toml");
    
    let partitions = r#"
# Name,   Type, SubType, Offset,  Size, Flags
nvs,      data, nvs,     0x9000,  0x6000,
otadata,  data, ota,     0xf000,  0x2000,
phy_init, data, phy,     0x11000, 0x1000,
factory,  app,  factory, 0x12000, 0x100000,
"#;
    std::fs::write(out_dir.join("partitions.csv"), partitions).unwrap();
    
    // Generate vial.json from keyboard.toml
    let vial_json = r#"{"keyboard_name":"keeboard","vendor_id":0x4b45,"product_id":0x4244,"matrix":{"rows":6,"cols":21},"layers":3,"encoders":[]}"#;
    std::fs::write(out_dir.join("vial.json"), vial_json).unwrap();
}