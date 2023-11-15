#!/bin/bash

set -xe

source scripts/.env

docker build -t vilchain:latest .

docker run --env-file scripts/.env vilchain