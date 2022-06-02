#![feature(new_uninit)]

use blake3::{
  guts::{parent_cv, ChunkState, CHUNK_LEN},
  Hash,
};

use std::io::{Error, Write};

#[derive(Debug, Clone)]
pub struct HashDepth {
  hash: Hash,
  depth: u8,
}

// (1<<10) * 1024 = 1MB
pub const BLOCK_CHUNK: u8 = 2;

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
    let mut len = self.li.len();
    let end = len == 0;
    if self.pos != 0 {
      self.push(true);
      len = self.li.len();
    } else if end {
      return;
    }

    let li = &mut self.li;
    len -= 1;
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
  }

  pub fn blake3(&self) -> Hash {
    let li = &self.li;
    let len = li.len();
    match len {
      0 => ChunkState::new(0).update(&[]).finalize(true),
      1 => li[0].hash,
      2 => parent_cv(&li[0].hash, &li[1].hash, true),
      len => {
        let mut n = 1;
        while li[n].depth == BLOCK_CHUNK {
          n += 1;
          if n == len {
            break;
          }
        }
        if n % 2 != 0 {
          n -= 1;
        }

        let mut hash_li: Vec<_> = li[n..].iter().rev().map(|i| i.hash).collect();
        let mut hash_li_len = len - n;

        if n != 0 {
          n = (n + 1) / 2;
          let mut li = {
            let mut box_li = unsafe { Box::<[Hash]>::new_uninit_slice(n).assume_init() };
            let mut i = 0;
            while i < n {
              let t = 2 * i;
              box_li[i] = parent_cv(&li[t].hash, &li[t + 1].hash, false);
              i += 1;
            }
            box_li
          };

          if n == 1 {
            hash_li.push(li[0]);
          }
        }

        let len = hash_li.len() - 1;

        n = 0;
        let right = &hash_li[0];
        n += 1;
        let mut finalize = len == n;

        let mut hash = parent_cv(&hash_li[n], right, finalize);

        while !finalize {
          n += 1;
          finalize = len == n;
          hash = parent_cv(&hash_li[n], &hash, finalize);
        }

        hash
      }
    }
  }

  fn push(&mut self, finalize: bool) {
    let li = &mut self.li;
    let mut len = li.len();
    let mut hash = self.state.finalize(finalize && len == 0);

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
    let mut remain = CHUNK_LEN - pos;
    let mut begin = 0;

    while begin < len {
      if remain == 0 {
        self.push(false);
        n += 1;
        self.state = ChunkState::new(n);
        pos = 0;
        remain = CHUNK_LEN;
      }
      let diff = len - begin;
      if diff < remain {
        pos += diff;
        self.state.update(&buf[begin..]);
        break;
      } else {
        let end = begin + remain;
        self.state.update(&buf[begin..end]);
        begin = end;
        remain = 0;
        pos = CHUNK_LEN;
      }
    }
    self.pos = pos;
    self.n = n;
    Ok(len)
  }

  fn flush(&mut self) -> Result<(), Error> {
    Ok(())
  }
}
