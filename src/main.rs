use r64drive::ftdi::R64DriveFtdi;
use r64drive::R64Drive;

fn main() {
    let driver = R64DriveFtdi::new();
    let r64d = R64Drive::new(&driver);
    let (variant, version) = r64d.get_version().expect("get_version error");
    println!("variant: {:?}, version: {}", variant, version);
}
