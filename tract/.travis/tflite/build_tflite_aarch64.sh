#!/bin/sh

set -ex
mkdir -p result

docker build -f Dockerfile.tensorflow-aarch64 --tag tensorflow-aarch64 .
docker run --rm -it \
    -v `pwd`/result:/result \
    tensorflow-aarch64 \
    sh -c "
         cd /tensorflow_src ;
         export EXTRA_CXXFLAGS=-flax-vector-conversions 
         export DISABLE_NNAPI=true
         ./tensorflow/lite/tools/make/download_dependencies.sh
         make -j 3 -f tensorflow/lite/tools/make/Makefile TARGET=linux TARGET_ARCH=aarch64 ;
         cp /tensorflow_src/tensorflow/lite/tools/make/gen/linux_aarch64/bin/benchmark_model /result/tflite_benchmark_model_aarch64
     "
