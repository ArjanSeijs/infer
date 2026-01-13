#!/bin/bash

file="$1"
filebase=$(basename $file)
line=$(head -n 1 $file)
outdir="${file//\//-}"
if [[ $line == //!!* ]]; then
    args=${line#//!!}
else
    args=""
fi
echo "Args: $args"

mkdir -p ./out/$outdir

cd ./out/$outdir
    echo $args
    charon rustc --print-ullbc $args -- ../../$file
    infer -g --enable-issue-type CONSTANT_ADDRESS_DEREFERENCE -- rustc $args -- ../../$file

cd ./infer-out/captured
    open $filebase.*.html || open charon.*.html

cd ../tmp
    if compgen -G "*.sil" > /dev/null; then subl *.sil_1.*.sil; fi

exit 0


echo "File:" $filebase