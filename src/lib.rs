#![feature(new_uninit)]

use blake3::{
  guts::{parent_cv, ChunkState, CHUNK_LEN},
  Hash,
};

use std::io::{Error, Read};

#[derive(Debug)]
pub struct HashDepth {
  hash: Hash,
  depth: u8,
}

// (1<<10) * 1024 = 1MB
pub const BLOCK_CHUNK: u8 = 10;

#[derive(Debug, Default)]
pub struct Merkle {
  pub li: Vec<HashDepth>,
}

impl Merkle {
  pub fn finalize(&mut self) {
    let li = &mut self.li;
    let len = li.len();
    let mut len = len - 1;
    let mut hash = li[len].hash;

    while len > 0 {
      len -= 1;
      let left = &li[len];
      if left.depth == BLOCK_CHUNK {
        len += 1;
        li[len].hash = hash;
        li.truncate(len + 1);
        break;
      }
      hash = parent_cv(&left.hash, &hash, 0 == len);
    }
    dbg!(&li);
  }

  pub fn blake3(&self) -> Hash {
    let li = &self.li;
    let len = li.len();
    match len {
      0 => ChunkState::new(0).update(&[]).finalize(true),
      1 => li[0].hash,
      2 => parent_cv(&li[0].hash, &li[1].hash, true),
      n => {
        let mut hash_li =
          unsafe { Box::<[Hash]>::new_uninit_slice((n / 2) + (n % 2)).assume_init() };

        hash_li[0] = parent_cv(&li[0].hash, &li[1].hash, false);
        hash_li[1] = parent_cv(&li[2].hash, &li[3].hash, false);
        let hash3 = parent_cv(&hash_li[0], &hash_li[1], true);
        hash3
      }
    }
  }

  pub fn push(&mut self, state: ChunkState, finalize: bool) {
    let mut hash = state.finalize(finalize);
    let li = &mut self.li;

    let mut len = li.len();
    let mut depth = 0;
    while len > 0 {
      len -= 1;
      let left = &li[len];
      if left.depth == depth {
        depth += 1;
        hash = parent_cv(&left.hash, &hash, finalize && len == 0);
        li.pop();
        if depth == BLOCK_CHUNK {
          break;
        }
      }
    }
    li.push(HashDepth { depth, hash });
  }
}

pub fn merkle(mut input: impl Read) -> Result<Merkle, Error> {
  let mut merkle = Merkle::default();
  dbg!(CHUNK_LEN);
  let mut buf: [u8; CHUNK_LEN] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
  let mut n: u64 = 0;

  #[allow(invalid_value)]
  let mut state: ChunkState = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

  macro_rules! push {
    ($finalize:expr) => {
      merkle.push(state, $finalize);
    };
  }

  loop {
    match input.read(&mut buf)? {
      CHUNK_LEN => {
        if n > 0 {
          push!(false);
        }
        state = ChunkState::new(n);
        state.update(&buf);
        n += 1;
      }
      0 => {
        if n > 0 {
          push!(n == 1);
        } else {
          state = ChunkState::new(0);
          state.update(&[]);
          push!(true);
        }
        break;
      }
      readed => {
        let is_root = n == 0;
        if !is_root {
          push!(false);
        }
        state = ChunkState::new(n);
        state.update(&buf[..readed]);
        push!(is_root);
        break;
      }
    }
  }
  merkle.finalize();
  Ok(merkle)
}
