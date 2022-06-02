#!/usr/bin/env bash

set -e
DIR=$( dirname $(realpath "$0") )
source $DIR/pid.sh

npm run prepare

NODE_ENV=development exec $DIR/coffee $@
