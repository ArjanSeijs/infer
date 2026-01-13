file="$1"
filebase=$(basename $file)
line=$(head -n 1 $file)
outdir="${file//\//-}"

mkdir -p ./out/$outdir
cd ./out/$outdir
if [[ "$1" = /* ]]; then
    PTH=$1
else 
    PTH=../../$file
fi
infer -g --pulse-only --capture-textual $PTH
cd ./infer-out/captured
open *.html