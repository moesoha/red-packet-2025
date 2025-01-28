# Deploy Step

```
qemu-system-x86_64 -nodefaults -m 48M \
	-drive if=pflash,unit=0,format=raw,file=OVMF_CODE.fd,readonly=on \
	-drive if=pflash,unit=1,format=raw,file=OVMF_VARS.fd \
	-usb -device usb-storage,drive=esp -drive id=esp,file=fat:rw:./EFI,format=raw,if=none \
	-vga virtio \
	-monitor vc:1280x800
```

1. put `Hongbao2025Setter.efi` into EFI folder, start qemu
2. run `Hongbao2025Setter.efi <hongbao-code>` in EFI Shell, exit
3. put `Hongbao2025Checker.efi` into EFI folder, start qemu
4. run `Hongbao2025Checker.efi` in EFI Shell
5. dump memory with `pmemsave 0 0x3000000 memdump.bin`
