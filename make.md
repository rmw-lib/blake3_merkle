# 基于 blake3 的 merkle tree

[blake3](https://github.com/BLAKE3-team/BLAKE3) 是基于 merkle tree 实现的，但是暴露的接口无法导出 merkle tree 。

[bao](https://github.com/oconnor663/bao) 实现了流式 blake3 验证，但不支持调整底层块大小 ( [support larger "chunk groups" for reduced space overhead](https://github.com/oconnor663/bao/issues/34) ) 。

bao 目前的实现会消耗 6% 的额外存储空间来记录验证哈希，对内容索引服务器来说，这是挺大的开销。

而 `blake3_merkle` ，当 `BLOCK_CHUNK` 设置为 10 时， 每 (1<<10)*1024 = 1MB 会输出一个 32 字节的哈希，只会增加 0.3‱  的额外开销。

`./examples/main.rs` 如下 :

```rust
#include ./examples/main.rs
```

运行 `./example.main.sh`，输出如下

```
#include ./main.out
```
