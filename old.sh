#!/bin/bash

file=$(basename $1)
args=${@:2}

mkdir -p ./out/$1

cd ./out/$1
    # echo "infer -g --pulse-only -- rustc --skip-borrowck -- ~/Files/Studie/Thesis/examples/$1" ${@:2}
    # infer -g --pulse-only -- rustc --skip-borrowck --print-ullbc --monomorphize -- ~/Files/Studie/Thesis/examples/$1 ${@:2}
    charon rustc --skip-borrowck --print-ullbc --include=core::ptr::null --include=core::ptr::null_mut --exclude=core -- ~/Files/Studie/Thesis/examples/$1 ${@:2}
    infer -g --pulse-only -- rustc --skip-borrowck --print-ullbc --include=core::ptr::null --include=core::ptr::null_mut --include=core::mem::drop --exclude=core -- ~/Files/Studie/Thesis/examples/$1 ${@:2}
    # infer -g --pulse-only -- rustc ~/Files/Studie/Thesis/examples/$1

cd ./infer-out/captured
    open $file.*.html || open charon.*.html
cd ../tmp
    if compgen -G "*.sil" > /dev/null; then subl *.sil_1.*.sil; fi

exit 0