#!/bin/bash
# from https://unix.stackexchange.com/a/629494

# Create a 2 MB file
dd if=/dev/zero of=disk.img bs=1M count=2

# Put a FAT filesystem on it (use -F for FAT32, otherwise it's automatic)
mformat -i disk.img ::

# Add a file to it
mcopy -i disk.img test.txt ::

# List files
mdir -i disk.img ::

# Test: Extract a file
#mcopy -i disk.img ::/test.txt extracted.txt