use blake3::guts::CHUNK_LEN;
use blake3_merkle::{Merkle, BLOCK_CHUNK};
use std::{error::Error, io::Write};

const BLOCK_SIZE: usize = (1 << BLOCK_CHUNK) * CHUNK_LEN;

fn test_blake3_merkle(len: usize) -> Result<(), Box<dyn Error>> {
  let bin: Vec<u8> = (0..len).map(|_| rand::random()).collect();
  let mut blake3 = blake3::Hasher::new();
  blake3.update(&bin);
  let mut merkle = Merkle::new();
  let _ = merkle.write(&bin)?;
  merkle.finalize();
  let true_hash = blake3.finalize();
  if merkle.blake3() != true_hash {
    dbg!(len, merkle.li);
    dbg!(true_hash);
    panic!();
  }
  if len <= BLOCK_SIZE && len > 0 {
    if merkle.li[0].hash != true_hash || merkle.li.len() != 1 {
      dbg!(true_hash);
      dbg!(&merkle.li[0]);
      panic!();
    }
  }
  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  //test_blake3_merkle(1033462)?;
  //return Ok(());

  for n in 0..2049 {
    dbg!(n);
    test_blake3_merkle(n)?;
    let base = n * CHUNK_LEN;
    for len in [base, base + 1, base + (rand::random::<u8>() as usize)] {
      test_blake3_merkle(len)?;
    }
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
