#!/bin/bash
set -eux

mv /usr/include /usr/include-bak | true

cd usr

gcc -c *.c
ar rcs libc.a *.o
mv libc.a lib

cd ../dash

export CFLAGS="--sysroot=/root/env/userland/ -I/root/env/userland/usr/include/"
export CPPFLAGS="--sysroot=/root/env/userland/ -I/root/env/userland/usr/include/"
export CPPFLAGS_FOR_BUILD="-I/root/env/userland/usr/include/"

export LDFLAGS="-L/root/env/userland/usr/lib -static -nostdlib -fno-builtin -Wl,--trace"
export LOCAL_LDFLAGS="-L/root/env/userland/usr/lib -static -nostdlib -fno-builtin -Wl,--trace"
export LDFLAGS_FOR_BUILD="-L/root/env/userland/usr/lib -static -nostdlib -fno-builtin -Wl,--trace"

if [ $# = 1 ] && [ "$1" = "-c" ]; then
	#./configure --host=x86_64-jos --prefix=/usr --without-bash-malloc --enable-static-link --disable-threads
	./configure --host=x86_64-jos --enable-static
fi

make clean 
make