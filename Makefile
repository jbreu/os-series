x86_64_boot_source_files := $(shell find src/bootloader -name *.asm)
x86_64_boot_object_files := $(patsubst src/bootloader/%.asm, build/x86_64/%.o, $(x86_64_boot_source_files))

$(x86_64_boot_object_files): build/x86_64/%.o : src/bootloader/%.asm
	mkdir -p $(dir $@) && \
	nasm -g -f elf64 -F dwarf $(patsubst build/x86_64/%.o, src/bootloader/%.asm, $@) -o $@

x86_64_asm_source_files := $(shell find src/kernel/asm -name *.asm)
x86_64_asm_object_files := $(patsubst src/kernel/asm/%.asm, build/x86_64/%.o, $(x86_64_asm_source_files))

$(x86_64_asm_object_files): build/x86_64/%.o : src/kernel/asm/%.asm
	mkdir -p $(dir $@) && \
	nasm -g -f elf64 -F dwarf $(patsubst build/x86_64/%.o, src/kernel/asm/%.asm, $@) -o $@

# TODO --allow-multiple-definition is a hack to have two static libs (kernel and helloworld) both with a panic handler. As helloworld should be a separate executable this will not be required
.PHONY: build-x86_64
build-x86_64: $(x86_64_boot_object_files) $(x86_64_asm_object_files)
	mkdir -p build/kernel && \
	cargo rustc --manifest-path src/kernel/Cargo.toml --target-dir build/kernel/ -- -C code-model=large -C no-redzone=on -C target-feature=-sse --target x86_64-unknown-none
	cargo rustc --manifest-path src/userland/Cargo.toml --target-dir build/userspace/ -- -C no-redzone=on -C target-feature=-sse --target x86_64-unknown-none
	mkdir -p dist/x86_64 && \
	x86_64-elf-ld -o dist/x86_64/kernel.bin --unresolved-symbols=report-all --allow-multiple-definition -z noexecstack -T targets/x86_64/linker.ld $(x86_64_boot_object_files) $(x86_64_asm_object_files) build/kernel/debug/libjos.a build/userspace/debug/libhelloworld.a && \
	cp dist/x86_64/kernel.bin targets/x86_64/iso/boot/kernel.bin && \
	grub-mkrescue /usr/lib/grub/i386-pc -o dist/x86_64/kernel.iso targets/x86_64/iso
