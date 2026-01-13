PTH="${1//\:\:/_}"

mkdir -p ./out/$PTH
cd ./out/$PTH

# charon rustc --start-from=core::array --include=core::array --print-ullbc -- ../../codebases/stdlib.rs --crate-type=lib
echo charon rustc --start-from=core::$1 --include=core::$1 ${@:2} --print-ullbc -- ../../codebases/stdlib.rs --crate-type=lib
infer -g --pulse-only -- rustc --start-from=core::$1 --include=core::$1 ${@:2} --print-ullbc -- ../../codebases/stdlib.rs --crate-type=lib
cd ./infer-out/
find ./captured -type f -name \*.html -maxdepth 1 -exec xdg-open {} \;
find ./tmp -maxdepth 1 -type f -name \*.sil  -exec xdg-open {} \;
exit 0