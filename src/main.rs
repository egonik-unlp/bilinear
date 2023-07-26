
pub mod manipulation;
use crate::manipulation::interface::EEMMatrix;

fn main() {
    let matrix = EEMMatrix::load_eem("rondo_acapricio.dat").unwrap();
    println!("{:?}", matrix.emission)
}