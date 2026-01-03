#!/bin/bash 
#set exec context to script directory
cd $(dirname "$(readlink -f "$0")")
#load env
[ ! -f .env ] || export $(grep -v '^#' .env | xargs)
#run app
./target/release/wallp
