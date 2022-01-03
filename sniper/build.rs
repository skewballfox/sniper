use prost_build;

fn main() {
    prost_build::compile_protos(&["../proto/sniper_service.proto"], &["../proto"]).unwrap();
    //println!("yeet");
}
