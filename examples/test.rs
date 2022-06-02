use blake3::guts::CHUNK_LEN;
use blake3_merkle::Merkle;
use std::{error::Error, io::Write};

fn test_blake3_merkle(len: usize) -> Result<(), Box<dyn Error>> {
  let bin: Vec<u8> = (0..len).map(|_| rand::random()).collect();
  let mut blake3 = blake3::Hasher::new();
  blake3.update(&bin);
  let mut merkle = Merkle::new();
  merkle.write(&bin)?;
  merkle.finalize();
  let true_hash = blake3.finalize();
  if merkle.blake3() != true_hash {
    dbg!(len, merkle.li);
    dbg!(true_hash);
    panic!();
  }
  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  //test_blake3_merkle(14337)?;
  //return Ok(());

  for n in 0..2049 {
    dbg!(n);
    test_blake3_merkle(n)?;
    let base = n * CHUNK_LEN;
    for len in [base, base + 1, base + (rand::random::<u16>() as usize)] {
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
