use blake3_merkle::merkle;
use std::{
  error::Error,
  fs::File,
  io::{copy, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
  let fpath = "/Users/z/Downloads/1.pdf";

  let mut hasher = blake3::Hasher::new();
  copy(&mut File::open(&fpath)?, &mut hasher)?;
  dbg!(hasher.finalize());

  let f = File::open(&fpath)?;
  dbg!(merkle(BufReader::new(f))?.blake3());

  Ok(())
}
