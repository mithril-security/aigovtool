import click
import threading
import queue 
from blindai.core import *
from drm import drm_server, enclave_counter



def run_server(out_remote_attestation_status, in_server_status):

    app = drm_server()
    in_server_status.put("up")
    app.run(host="0.0.0.0", port="6000", ssl_context=('./localhost.crt', './localhost.key'))
    remote_attestation_status = False
    while not remote_attestation_status:
        data = out_remote_attestation_status.get()
        if data == "success":
            remote_attestation_status = True
    click.echo("Connected and model uploaded.")
    click.echo("starting the DRM server...")

    

def connect_inference(address, upload, in_remote_attestation_status, out_server_status):
    click.echo("Connecting to the Inference Server...") 
    client_v2 = connect(addr=address, hazmat_http_on_unattested_port=True) # TODO: error handling in case RA failed
    click.echo("Inference server remote attestation completed successfully.")
    click.echo(f"Sending the model {upload}...")
    response = client_v2.upload_model(model=upload)
    click.echo(response)
    in_remote_attestation_status.put("success")
    server_status = False
    while not server_status:
        data = out_server_status.get()
        print(data)
        if data == "up":
            server_status = True
    click.echo("sending up status.")
    response = client_v2.send_status()
    click.echo(response.content)


@click.command()
@click.option("--address", prompt="Inference server to connect to (format : domain or IP)", default="127.0.0.1", help='Domain or IP of the inference server. (Default Port for blindai)', type=str)
@click.option("--upload", prompt="Path to the AI model", default=1, help='Path to upload your AI Model (ONNX format).', type=str)
def start(address, upload):
    remote_attestation_status = queue.Queue()
    server_status = queue.Queue()
    threading.Thread(target=connect_inference, args=(address, upload, remote_attestation_status, server_status)).start()
    threading.Thread(target=run_server, args=(remote_attestation_status, server_status)).start()



if __name__ == '__main__':
    start()
