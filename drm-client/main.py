import click
from blindai.core import *

def model_acquire(address):
    click.echo("Connecting to the blindAI server.")
    client_v2 = connect(addr=address, hazmat_http_on_unattested_port=True) # TODO: error handling in case RA failed
    click.echo("Inference server remote attestation completed successfully.")
    click.echo("requesting the available model ID.") 
    models_available = client_v2.get_available_models()
    models_available = models_available.models_info
    return models_available

    
class ModelInfo: 
    model_id: str
    model_name: str

    def __init__(self, model_id, model_name):
        self.model_id = model_id
        self.model_name = model_name


@click.command()
@click.option("--address", prompt="Inference server to connect to (format : domain or IP)", default="127.0.0.1", help='Domain or IP of the inference server.(Default Port for Blindai)', type=str)
def start(address):
    models_available = model_acquire(address)

    # for model in models_available:
    #     models.append(ModelInfo(model.model_id, model.model_name))
    models = []
    click.echo("The available models in store are : ")
    for model in models_available:
        click.echo(f'{model.model_name} : {model.model_id}')
        models.append({'model_name':model.model_name, 'model_id':model.model_id })

    model_run = click.prompt('Which model to run?', type=click.Choice([model['model_name'] for model in models]))
    print(model_run)
if __name__ == '__main__':
    start()