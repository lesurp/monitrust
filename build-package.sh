#!/bin/bash

outdir=package
cur_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

mkdir -p $outdir
podman build . --output dest=$outdir,type=local --target package
