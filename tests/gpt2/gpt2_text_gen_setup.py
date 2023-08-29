import torch
from transformers import GPT2Tokenizer

tokenizer = GPT2Tokenizer.from_pretrained('gpt2')
tokenizer.padding_side = "left"
tokenizer.pad_token = tokenizer.eos_token
max_length = 64
num_attention_heads, hidden_size, num_layer = 12, 768, 12

def get_example_inputs(prompt_text):
    encodings_dict = tokenizer.batch_encode_plus(prompt_text, padding='max_length', max_length=64)

    input_ids = torch.tensor(encodings_dict["input_ids"], dtype=torch.int32)
    attention_mask = torch.tensor(encodings_dict["attention_mask"], dtype=torch.int32)
    position_ids = attention_mask.long().cumsum(-1) - 1
    position_ids.masked_fill_(position_ids < 0, 0)
    position_ids = position_ids.to(torch.int32)

    # Empty Past State for generating first word
    empty_past = []
    batch_size = input_ids.size(0)
    sequence_length = input_ids.size(1)
    past_shape = [2, batch_size, num_attention_heads, 0, hidden_size // num_attention_heads]
    for i in range(num_layer):
        empty_past.append(torch.empty(past_shape).type(torch.float32).to(device))

    return input_ids, attention_mask, position_ids, empty_past

import onnxruntime

ort_meta = onnxruntime.InferenceSession("gpt2_text_gen.onnx")

example = "I like the Rust programming language because"
device = "cpu"
input_ids, attention_mask, position_ids, past = get_example_inputs([example])

def to_numpy(tensor):
    return tensor.detach().cpu().numpy() if tensor.requires_grad else tensor.cpu().numpy()

ort_inputs = {ort_meta.get_inputs()[0].name: to_numpy(input_ids), 
              ort_meta.get_inputs()[1].name: to_numpy(attention_mask), 
              ort_meta.get_inputs()[2].name: to_numpy(position_ids), 
              }

for i in range(len(past)):
    ort_inputs[ort_meta.get_inputs()[i+3].name] = to_numpy(past[i])

import numpy as np

np.savez("./gpt2_text_gen.npz", **ort_inputs)
print(ort_inputs)