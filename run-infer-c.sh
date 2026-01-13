file="$1"
filebase=$(basename $file)
line=$(head -n 1 $file)
outdir="${file//\//-}"


mkdir -p ./out/$outdir
cd ./out/$outdir
    infer -g --pulse-only --enable-issue-type CONSTANT_ADDRESS_DEREFERENCE -- clang -c ../../$file
cd ./infer-out/captured
    open *.html
exit 0