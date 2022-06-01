use blake3_merkle::merkle;
use std::{
  error::Error,
  fs::File,
  io::{copy, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
  let fpath = "/Users/z/Downloads/1.pdf";

  let mut blake3 = blake3::Hasher::new();
  copy(&mut File::open(&fpath)?, &mut blake3)?;

  let f = File::open(&fpath)?;
  let merkle = merkle(BufReader::new(f))?;
  dbg!(&merkle);
  dbg!(merkle.blake3());
  dbg!(blake3.finalize());

  Ok(())
}
