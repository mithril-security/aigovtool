from flask import Flask, request, jsonify
import click
import threading
import queue 
import time
import json

from blindai.core import *
from drm import drm_server




def enclave_ready_listener(address, upload):
    app = Flask(__name__)

    @app.route('/enclave_ready', methods=['GET'])
    def enclave_ready():
        if request.method == 'GET':
            click.echo('Enclave ready to connect.')
            click.echo("Starting the main process.")
            remote_attestation_status = queue.Queue()
            server_status = queue.Queue()
            threading.Thread(target=connect_inference, args=(address, upload, remote_attestation_status, server_status)).start()
            threading.Thread(target=run_server, args=(remote_attestation_status, server_status)).start()
            return  {"status" : "Received, begining connection."}
        else:
            return {"error" : "Method not allowed"}
        
    return app
        
def start_enclave_listener(address: str, upload: str, num_requests: int):
    with open('inferences.json', 'w') as f:
        json.dumps({"inferences": str(num_requests)}, f)
    print(f"number of inferences set up at : {num_requests}")   
    app_r = enclave_ready_listener(address, upload=upload)
    app_r.run(host="0.0.0.0", port="7000", ssl_context=('./localhost.crt', './localhost.key'))


def run_server(out_remote_attestation_status, in_server_status):
    time.sleep(2)
    remote_attestation_status = False
    while not remote_attestation_status:
        data = out_remote_attestation_status.get()
        if data == "success":
            remote_attestation_status = True
    click.echo("Connected and model uploaded.")
    click.echo("starting the DRM server...")
    app = drm_server()
    in_server_status.put("up")
    app.run(host="0.0.0.0", port="6000", ssl_context=('./localhost.crt', './localhost.key'))

# def run_server_mod(in_server_status):
#     app =drm_server()
#     in_server_status.put("up")
#     app.run(host="0.0.0.0", port="6000", ssl_context=('./localhost.crt', './localhost.key'))
#     state = app.enclave_ready()

def connect_inference(address, upload, in_remote_attestation_status, out_server_status):
    click.echo("Connecting to the Inference Server...") 
    client_v2 = connect(addr=address, hazmat_http_on_unattested_port=True) # TODO: error handling in case RA failed
    click.echo("Inference server remote attestation completed successfully.")
    click.echo(f"Sending the model {upload}...")
    time.sleep(2)
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
@click.option("--upload", prompt="Path to the AI model", default='./', help='Path to upload your AI Model (ONNX format).', type=str)
@click.option("--num_requests", prompt="Number of requests granted to the user", default=10, help='Number of requests.', type=int)
def start(address, upload, num_requests):
    start_enclave_listener(address=address, upload=upload, num_requests=num_requests)


if __name__ == '__main__':
    start()
