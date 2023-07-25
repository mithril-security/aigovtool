
run_in_tf_docker() {
    docker run --rm -v $HOME/.cache:/models -it tensorflow/tensorflow:nightly-devel sh -c "$@"
}

# # inception v3 
# run_in_tf_docker "cd /models ; tflite_convert \
#     --graph_def_file inception_v3_2016_08_28_frozen.pb \
#     --input_arrays input \
#     --input_shapes 1,299,299,3 \
#     --output_arrays InceptionV3/Predictions/Reshape_1 \
#     --output_format tflite \
#     --output_file inception_v3_2016_08_28_frozen.tflite"
# 
# # arm ml kws
# run_in_tf_docker "cd /models ; tflite_convert \
#     --graph_def_file ARM-ML-KWS-CNN-M.pb \
#     --input_arrays Mfcc \
#     --input_shapes 1,49,10 \
#     --output_arrays labels_softmax \
#     --output_format tflite \
#     --output_file ARM-ML-KWS-CNN-M.tflite"

# hey_snips v1
run_in_tf_docker "cd /models ; tflite_convert \
    --graph_def_file hey_snips_v1.pb \
    --input_arrays inputs \
    --input_shapes 80,40 \
    --output_arrays logits \
    --output_format tflite \
    --output_file hey_snips_v1.tflite"

# hey_snips v3.1
# (tflite does not support 1D dil)
# run_in_tf_docker "cd /models ; tflite_convert \
#     --graph_def_file hey_snips_v3.1.pb \
#     --input_arrays inputs \
#     --input_shapes 40,40 \
#     --output_arrays logits \
#     --output_format tflite \
#     --output_file hey_snips_v3.1.tflite"
# 
# # hey_snips v4 model17, 2seconds
# (tflite does not support AddN)
# run_in_tf_docker "cd /models ; tflite_convert \
#     --graph_def_file hey_snips_v4_model17.pb \
#     --input_arrays input_node \
#     --input_shapes 200,20 \
#     --output_arrays output_node \
#     --output_format tflite \
#     --output_file hey_snips_v4_model17.tflite"
