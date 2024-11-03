#!/bin/bash

mv /usr/include /usr/include-bak

cd usr

gcc -c *.c
ar rcs libc.a *.o

cd ../bash

export LDFLAGS="--sysroot=/root/env/userland/ -L/root/env/build/userspace/x86_64-unknown-none/debug/libc.a -static -nostdlib -fno-builtin -Wl,--trace"
export LOCAL_LDFLAGS="--sysroot=/root/env/userland/ -L/root/env/build/userspace/x86_64-unknown-none/debug/libc.a -static -nostdlib -fno-builtin -Wl,--trace"
export CFLAGS="--sysroot=/root/env/userland/ -I/root/env/userland/usr/include/"
export CPPFLAGS="--sysroot=/root/env/userland/ -I/root/env/userland/usr/include/"
export CPPFLAGS_FOR_BUILD="-I/root/env/userland/usr/include/"

# x86_64-jos: to make it work, add everywhere in configure, config.sub and config.guess where haiku is mentioned a similar entry for jos

./configure --host=x86_64-jos --prefix=/usr --without-bash-malloc --disable-threads

make clean
make 