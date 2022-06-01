use blake3::guts::CHUNK_LEN;
use blake3_merkle::Merkle;
use rand::Rng;
use std::{
  error::Error,
  fs::File,
  io::{copy, Write},
};

fn test_blake3_merkle(bin: &[u8]) -> Result<(), Box<dyn Error>> {
  let mut blake3 = blake3::Hasher::new();
  blake3.update(bin);
  let mut merkle = Merkle::new();
  merkle.write(bin)?;
  merkle.finalize();
  if merkle.blake3() != blake3.finalize() {
    dbg!(bin.len(), merkle.li);
  }
  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  for len in [0, 1, CHUNK_LEN / 2, CHUNK_LEN - 1, CHUNK_LEN, CHUNK_LEN + 1] {
    let bin: Vec<u8> = (0..len).map(|_| rand::random::<u8>()).collect();
    test_blake3_merkle(&bin)?;
  }

  /*
  let fpath = "/Users/z/Downloads/1.pdf";

  let mut blake3 = blake3::Hasher::new();
  copy(&mut File::open(&fpath)?, &mut blake3)?;

  let mut merkle = Merkle::new();
  copy(&mut File::open(&fpath)?, &mut merkle)?;
  merkle.finalize();
  dbg!(&merkle.li);
  dbg!(blake3.finalize());
  */
  Ok(())
}
