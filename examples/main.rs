use blake3::guts::CHUNK_LEN;
use blake3_merkle::Merkle;
use rand::Rng;
use std::{
  error::Error,
  fs::File,
  io::{copy, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
  let fpath = "/Users/z/Downloads/1.pdf";

  let mut blake3 = blake3::Hasher::new();
  copy(&mut File::open(&fpath)?, &mut blake3)?;

  let mut merkle = Merkle::new();
  copy(&mut File::open(&fpath)?, &mut merkle)?;
  merkle.finalize();
  dbg!(&merkle.li);
  dbg!(merkle.blake3());
  dbg!(blake3.finalize());
  Ok(())
}
