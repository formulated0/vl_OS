RUST_SRC = kernelmain.rs
RUST_OBJ = kernelmain.o
ASM_OBJ = loader.o
OBJECTS = $(ASM_OBJ) $(RUST_OBJ)

RUSTC = rustc
AS = nasm

RUSTFLAGS = --target i686-unknown-none \
            --edition 2024 \
            --emit obj \
            -C opt-level=2 \
            -C panic=abort \
            -C debuginfo=0 \
			-A unused

ASFLAGS = -f elf32
LDFLAGS = -T link.ld -melf_i386

all: kernel.elf
	mkdir -p iso/boot/grub
	cp grub.cfg iso/boot/grub/grub.cfg
	cp kernel.elf iso/boot/kernel.elf
	grub-mkrescue -o vl_OS.iso iso -d /usr/lib/grub/i386-pc

kernel.elf: $(OBJECTS)
	ld $(LDFLAGS) $(OBJECTS) -o kernel.elf

$(RUST_OBJ): $(RUST_SRC)
	rustc --edition 2024 --emit obj \
	    -C panic=abort \
	    -C overflow-checks=off \
	    -C opt-level=2 \
	    -C target-cpu=i686 \
	    -C target-feature=-mmx,-sse,-sse2 \
	    --target i686-unknown-linux-gnu \
	    $< -o $@
		
$(ASM_OBJ): loader.s
	$(AS) $(ASFLAGS) $< -o $@

.PHONY: all clean runqemu cleanrunqemu

runqemu: all
	qemu-system-i386 -cdrom vl_OS.iso

cleanrunqemu: all
	@trap '$(MAKE) clean' EXIT; \
	qemu-system-i386 -cdrom vl_OS.iso

clean:
	rm -rf *.o kernel.elf vl_OS.iso iso/