mkdir -p ./out/$1
file=$(basename $1)
cd ./out/$1
infer -g --pulse-only -- python3 ~/Files/Studie/Thesis/examples/$1
cd ./infer-out/captured
open $file.*.html
cd ../tmp
subl *.sil
exit 0