import click
import json
import numpy as np
from blindai.core import *
from torchvision import transforms
from PIL import Image
from matplotlib import pyplot as plt
import requests
import torch

# parses input to use it as a tensor
def process_input(input_data):
    input_image = Image.open(input_data)
    plt.imshow(input_image)
    plt.axis("off")

    # preprocessing function to resize image and turn into tensor
    preprocess = transforms.Compose(
        [
            transforms.Resize(256),
            transforms.CenterCrop(224),
            transforms.ToTensor(),
            transforms.Normalize(mean=[0.485, 0.456, 0.406], std=[0.229, 0.224, 0.225]),
        ]
    )
    input_tensor = preprocess(input_image)

    # create a mini-batch as expected by the model
    input_batch = input_tensor.unsqueeze(0)
    return input_batch


def process_predictions(prediction):
    # get labels as list
    response = requests.get("https://git.io/JJkYN")
    labels = response.text.split("\n")
    # Get results from RunModelResponse
    # output = prediction.output[0].as_torch()
    output = prediction
    # Find the score in terms of percentage by using torch.nn.functional.softmax function
    # which normalizes the output to range [0,1]
    probabilities = torch.nn.functional.softmax(output, dim=1)

    # get index of item with highest probability
    index = probabilities.argmax().item()

    # get label at this index
    print("Label is:", labels[index])

    # Get the probability- which is the highest probability from output
    print("Probability is:", probabilities.max().item())

def model_acquire(address):
    click.echo("Connecting to the blindAI server.")
    client_v2 = connect(addr=address, hazmat_http_on_unattested_port=True) # TODO: error handling in case RA failed
    click.echo("Inference server remote attestation completed successfully.")
    click.echo("requesting the available model ID.") 
    models_available = client_v2.get_available_models()
    models_available = models_available.models_info
    models = []
    model_to_run = {}
    if len(models_available) <= 0: 
        click.echo("There is no model uploaded in the store")
    else: 
        click.echo("The available models in store are : ")
        for model in models_available:
            click.echo(f'{model.model_name} : {model.model_id}')
            models.append({'model_name':model.model_name, 'model_id':model.model_id })
        while len(model_to_run) <= 0:
            model_run = click.prompt('Select the model you want to run :', type=click.Choice([model['model_name'] for model in models]))
            for i in range(len(models)):
                if model_run == models[i]['model_name']:
                    model_to_run = models[i]
                else:
                    click.echo("Model typed doesn't exist.")
    return client_v2, model_to_run

    

@click.command()
@click.option("--address", prompt="Inference server to connect to (format : domain or IP)", default="127.0.0.1", help='Domain or IP of the inference server.(Default Port for Blindai)', type=str)
@click.option("--input", prompt="Input that will be processed by the model and the inference server.", default="", help="Only tested for the ResNet Model. Please supply an image.", type=str)
def start(address, input):
    client_v2, model_to_run = model_acquire(address)
    inferences_left = client_v2.get_available_inferences()
    inferences_left = inferences_left.content.decode("utf-8")
    inferences_left = json.loads(inferences_left)
    input_batch = process_input(input)
    click.echo(f'The number of inferences left is {inferences_left["inferences"]}')
    while True:
        inferences_left = client_v2.get_available_inferences()
        inferences_left = inferences_left.content.decode("utf-8")
        inferences_left = json.loads(inferences_left)
        print(inferences_left)
        if int(inferences_left["inferences"])>0 :
            confirm_run = click.prompt("Run the model ? (R/n)")
            if confirm_run == "R":
                input_tensors=input_batch
                run_response= client_v2.run_model( model_id=model_to_run["model_id"],input_tensors=input_tensors)
                inference_results = run_response.output[0].as_numpy()
                process_predictions(torch.tensor(inference_results))
                # click.echo(f'Inference results : {inference_results}')
            else:
                click.echo("Not confirmed.")
        else:
            click.echo("Waiting for new consumption request.")
            input_tensors=input_batch
            run_response= client_v2.run_model( model_id=model_to_run["model_id"],input_tensors=input_tensors)
            inference_results = run_response.output[0].as_numpy()
            process_predictions(torch.tensor(inference_results))


            
if __name__ == '__main__': 
    start()