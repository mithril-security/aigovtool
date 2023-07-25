import click
from blindai.core import *
from drm import drm_server


@click.command()
@click.option("--address", prompt="Inference server to connect to (format : domain or IP)", default=1, help='Domain or IP of the inference server. (Default Port for blindai)', type=str)
@click.option("--upload", prompt="Path to the AI model", default=1, help='Path to upload your AI Model (ONNX format).', type=str)
def start(address, upload):
    click.echo("Connection to the Inference model...")
    client_v2 = connect(addr=address, hazmat_http_on_unattested_port=True)
    click.echo("Inference server remote attestation completed successfully.")
    click.echo(f"Sending the model {upload}...")
    response = client_v2.upload_model(model=upload)
    click.echo("Connected and model uploaded.")
    click.echo("starting the DRM server...")
    app = drm_server()
    app.run(host="0.0.0.0", port="6000", ssl_context=('./cert.pem', './key.pem'))

if __name__ == '__main__':
    start()
