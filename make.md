# 基于 blake3 的 merkle tree

[blake3](https://github.com/BLAKE3-team/BLAKE3) 的底层是一棵 merkle tree ，但是暴露的接口无法导出 merkle tree 。

[bao](https://github.com/oconnor663/bao) 实现了 blake3 流式验证，但无法调整底层块大小 ( [support larger "chunk groups" for reduced space overhead](https://github.com/oconnor663/bao/issues/34) ) 。

也就是说， bao 会消耗 6% 的额外存储空间来记录 merkle tree。对于分布式内容索引来说，这是挺大的开销。

于是，我实现了 [blake3_merkle](https://github.com/rmw-lib/blake3_merkle) ，当 `BLOCK_CHUNK` 设置为 10 时， 每 (1<<10)*1024 = 1MB 会输出一个 32 字节的哈希，只会增加 0.3‱  的额外开销。

`./examples/main.rs` 如下 :

```rust
#include ./examples/main.rs
```

运行 `./example.main.sh`，输出如下

```
#include ./main.out
```
