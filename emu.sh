#!/usr/bin/env bash

set -ex

  # -kernel arch/x86_64/boot/bzImage \
  # -kernel vmlinux \
  # -machine type=pc-i440fx-3.1 \
  # -m 4G \
  # -net user,hostfwd=tcp::10022-:22 \
  # -s -S
  # -initrd initrd.img \

sudo qemu-system-x86_64 \
  -kernel arch/x86_64/boot/bzImage \
  -initrd ramdisk.img \
  -display none \
  -serial stdio \
  -append "root=/dev/vda console=ttyS0 nokaslr noapic" \
  -drive format=raw,file=/home/gnzh/mydev/linsicles/buildroot/output/images/rootfs.ext4,if=virtio
