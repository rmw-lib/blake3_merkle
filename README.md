<!-- 本文件由 ./make.md 自动生成，请不要直接修改此文件 -->

# 基于 blake3 的 merkle tree

[blake3](https://github.com/BLAKE3-team/BLAKE3) 是基于 merkle tree 实现的，但是暴露的接口无法导出 merkle tree 。

[bao](https://github.com/oconnor663/bao) 实现了流式 blake3 验证，但不支持调整底层块大小 ( [support larger "chunk groups" for reduced space overhead](https://github.com/oconnor663/bao/issues/34) ) 。

bao 目前的实现会消耗 6% 的额外存储空间来记录验证哈希，对于 p2p 索引来说，这是挺大的开销。

`BLOCK_CHUNK` 设置为 10， 也就是 (1<<10)*1024 = 1MB 会输出一个 32 字节的哈希，只会增加 0.3‱  的额外开销。