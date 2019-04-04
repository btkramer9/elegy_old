from shutil import copyfile
from subprocess import call


call(['wasm-gc', 
      'target/wasm32-unknown-unknown/release/elegy.wasm', 
      'elegy.wasm'])