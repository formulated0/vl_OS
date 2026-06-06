# vl_OS

a tiny bootstrapped toy os for messing around with rust + multiboot

the state right now:

- `loader.s` boots the kernel
- `kernelmain.rs` prints simple text
- text framebuffer writes + cursor stuff works

todo:

- [ ] input handling
- [ ] make the kernel actually do something useful (`!!!`)
- [ ] implement a basic syscall / task switcher
- [ ] clean up build flow and cross-platform support

build:

- `make` creates `vl_OS.iso`
- `make runqemu` boots it in qemu

notes:

- pre-pre-alpha `! ! ! `
- no package manager gimmicks, just rustc + nasm + ld
