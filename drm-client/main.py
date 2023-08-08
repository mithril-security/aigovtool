import click
import json
import numpy as np
from blindai.core import *

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
def start(address):
    client_v2, model_to_run = model_acquire(address)
    inferences_left = client_v2.get_available_inferences()
    inferences_left = inferences_left.content.decode("utf-8")
    inferences_left = json.loads(inferences_left)
    click.echo(f'The number of inferences left is {inferences_left["inferences"]}')
    while int(inferences_left["inferences"]) >0:
        print(inferences_left)
        inferences_left = client_v2.get_available_inferences()
        inferences_left = inferences_left.content.decode("utf-8")
        inferences_left = json.loads(inferences_left)
        if int(inferences_left["inferences"]) >0 :
            confirm_run = click.prompt("Run the model ? (Y/n)")
            if confirm_run == "Y":
                input_tensors={"input": np.array(42), "sub": np.array(40)}
                run_response= client_v2.run_model( model_id=model_to_run["model_id"],input_tensors=input_tensors)
                inference_results = run_response.output[0].as_numpy()
                click.echo(f'Inference results : {inference_results}')
            else:
                click.echo("Not confirmed.")
if __name__ == '__main__': 
    start()