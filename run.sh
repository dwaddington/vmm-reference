#!/bin/bash
rust-gdb --args ./vmm-reference                                                            \
    --memory size_mib=12000                                          \
    --vcpu num=1                                                 \
    --kernel path=vmlinux,kernel_load_addr=1024,cmdline="pci=off"   \
    --block path=rootfs.ext4

    #[--block <blkdev_config> - TBD]
