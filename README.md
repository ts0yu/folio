# folio 📖
![Github Actions](https://github.com/primitivefinance/arbiter/workflows/Rust/badge.svg)
[![](https://dcbadge.vercel.app/api/server/primitive?style=flat)](https://discord.gg/primitive)
![](https://visitor-badge.laobi.icu/badge?page_id=ts0yu.folio)
## overview

folio provides a fast, simple abstraction and work-in-progress compiler for the [FVM](https://www.primitive.xyz/papers/yellow.pdf) (Financial Virtual Machine). 

## features

- [x] lexical analysis
- [x] parsing
- [x] basic code generation and bytecode output
- [ ] finalize code generation, and ensure parity with `FVM.ts`  

## compiler

example usage of the folio compiler can be found within the [`/examples`](https://github.com/ts0yu/folio/tree/main/examples) folder.

### structure

- crates
  - cli -> command line interface
  - compiler
    -  `assembler.rs` -> defines the parser
    -  `codegen.rs` -> code generation module
    -  `opcode/rs` -> defines a set of FVM primitive types
    -  `token.rs` -> lexical analysis and token definition

## contact

- [email](mailto:ts0yu@proton.me)
- [discord](https://discord.gg/primitive)
