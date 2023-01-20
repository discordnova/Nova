#!/bin/bash

N=4
i=0
TARGETS=(
   # Linux builds
   "linux-arm64-gnuc:aarch64-unknown-linux-gnu"
   "linux-arm64-musl:aarch64-unknown-linux-musl"
   "linux-armv7-gnuc:armv7-unknown-linux-gnueabi"
   "linux-armv7-musl:armv7-unknown-linux-musleabi"
   "linux-86_64-gnuc:x86_64-unknown-linux-gnu"
   "linux-86_64-musl:x86_64-unknown-linux-musl"

   # windows builds
   "windows-86_64-gnu:x86_64-pc-windows-gnu"
)

for thing in "${TARGETS[@]}"; do 
   KEY=${thing%%:*}
   VALUE=${thing#*:}
   
   echo "* BUILDING FOR $VALUE"
   cross build --release --target $VALUE

   # Copy intol folders
   mkdir -p ./build/$KEY/
   cp target/$VALUE/release/* ./build/$KEY/

   rm ./build/$KEY/*.{d,rlib}
done

wait < <(jobs -p)
