#![feature(new_uninit)]

use blake3::{
  guts::{parent_cv, ChunkState, CHUNK_LEN},
  Hash,
};

use std::{
  io::{Error, Read, Write},
  mem::replace,
};

#[derive(Debug)]
pub struct HashDepth {
  hash: Hash,
  depth: u8,
}

// (1<<10) * 1024 = 1MB
pub const BLOCK_CHUNK: u8 = 10;

#[derive(Debug)]
pub struct Merkle {
  pub li: Vec<HashDepth>,
  pub pos: usize,
  pub n: u64,
  pub state: ChunkState,
}

impl Merkle {
  pub fn new() -> Self {
    Merkle {
      li: vec![],
      pos: 0,
      n: 0,
      state: ChunkState::new(0),
    }
  }
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
    //dbg!(&li);
  }

  pub fn blake3(&self) -> Hash {
    let li = &self.li;
    let len = li.len();
    match len {
      0 => ChunkState::new(0).update(&[]).finalize(true),
      1 => li[0].hash,
      2 => parent_cv(&li[0].hash, &li[1].hash, true),
      len => {
        let mut hash_len = len / 2;
        let end = len % 2;
        let mut box_len = hash_len + end;
        let mut hash_li = unsafe { Box::<[Hash]>::new_uninit_slice(box_len).assume_init() };
        if end != 0 {
          hash_li[hash_len] = li[len - 1].hash;
        }

        while hash_len != 0 {
          hash_len -= 1;
          let t = hash_len * 2;
          hash_li[hash_len] = parent_cv(&li[t].hash, &li[t + 1].hash, false);
        }
        while box_len > 2 {
          let mut hash_len = box_len / 2;
          let end = box_len % 2;
          let len = hash_len + end;
          let mut li = unsafe { Box::<[Hash]>::new_uninit_slice(len).assume_init() };
          if end != 0 {
            li[hash_len] = hash_li[box_len - 1];
          }
          while hash_len != 0 {
            hash_len -= 1;
            let t = hash_len * 2;
            li[hash_len] = parent_cv(&hash_li[t], &hash_li[t + 1], false);
          }
          box_len = len;
          hash_li = li
        }

        parent_cv(&hash_li[0], &hash_li[1], true)
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

impl Write for Merkle {
  fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
    let len = buf.len();
    let mut pos = self.pos;
    let mut n = self.n;

    let begin = 0;

    while begin < len {
      let diff = len - begin;
      if diff < (CHUNK_LEN - pos) {
        pos += diff;
        self.state.update(&buf[begin..]);

        n += 1;
        let state = replace(&mut self.state, ChunkState::new(n));
        self.push(state, false);
        break;
      } else {
        n += 1;
        break;
      }
    }
    self.pos = pos;
    self.n = n;
    Ok(len)
  }

  fn flush(&mut self) -> Result<(), Error> {
    #[allow(invalid_value)]
    let state = replace(&mut self.state, unsafe {
      std::mem::MaybeUninit::uninit().assume_init()
    });
    self.push(state, true);
    Ok(())
  }
}

pub fn merkle(mut input: impl Read) -> Result<Merkle, Error> {
  let mut merkle = Merkle::new();
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
