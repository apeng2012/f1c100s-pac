#!/bin/bash

svd2rust -i f1c100s.svd --target none --keep_list

rm -rf src/

form -i lib.rs -o src/ 

rm lib.rs

cargo fmt
