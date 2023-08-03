import click
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
            model_run = click.prompt('Which model to run?', type=click.Choice([model['model_name'] for model in models]))
            for i in range(len(models)):
                if model_run == models[i]['model_name']:
                    model_to_run = models[i]
                else:
                    click.echo("Model typed doesn't exist.")
        print(model_to_run)

    

@click.command()
@click.option("--address", prompt="Inference server to connect to (format : domain or IP)", default="127.0.0.1", help='Domain or IP of the inference server.(Default Port for Blindai)', type=str)
def start(address):
    model_acquire(address)


if __name__ == '__main__':
    start()