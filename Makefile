build: src/*.rs
	cargo build --release
	dd if=/dev/zero of=snake_os.iso bs=512 count=93750
	mkfs.fat -F 32 snake_os.iso
	mkdir -p mnt/
	sudo mount snake_os.iso mnt/
	sudo mkdir -p mnt/EFI/Boot/
	sudo cp target/x86_64-unknown-uefi/release/snake_os.efi mnt/EFI/Boot/Bootx64.efi
	sudo umount -R mnt/
	rm -rf mnt/

run: build
	qemu-system-x86_64 -bios /usr/share/ovmf/x64/OVMF.fd \
	-object rng-random,filename=/dev/urandom,id=rng0 \
	-device virtio-rng-pci,rng=rng0 \
	snake_os.iso

clean:
	rm -rf target snake_os.iso Cargo.lock