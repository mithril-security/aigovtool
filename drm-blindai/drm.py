from flask import Flask, request, jsonify
import json



# DRM server, goals: 
#   - listening requests from the Inference Server for consumption budget
#   - sending each inference for tracing
def drm_server():
    app = Flask(__name__)

    @app.route('/supply_inferences', methods=['POST'])
    def supply_inferences():
        if request.method == 'POST':
            number_inferences = request.form['number_inferences']
            print("* CUSTODIAN REQ [POST: /request_consumption] Requesting new number of inferences : " + str(number_inferences))
            with open('inferences.json', 'w') as f: 
                json.dump({"inferences" : str(number_inferences)}, f)
            return {"inferences": number_inferences}
        else:
            return {"error" : "Method not allowed"}

    @app.route('/request_consumption', methods=['POST', 'GET'])
    def request_consumption():
        if request.method == 'POST':
            # we can imagine that it looks into a database to see if the number of inferences asked for 
            # is allowed
            number_inferences = request.form['number_inferences']
            print("* [POST: /request_consumption] Requesting new number of inferences : " + str(number_inferences))
            with open('inf_req.json', 'w') as f: 
                json.dump({"request_inference" : True}, f)
            return {"inferences": number_inferences}
        elif request.method == 'GET':
            json_inferences = {}
            with open('inferences.json', 'r') as f:
                json_inferences = json.load(f)
            print("* [GET: /request_consumption] Number of inferences remaining : " + str(json_inferences["inferences"]))
            return json_inferences
        else:
            return {"error" : "Method not allowed"}
        
    @app.route('/consume_model', methods=['POST'])
    def consume_model():
        if request.method == "POST":
            req = request.form['run_model']
            print("* [GET: /consume_model] Model running :" + req)
            number_inferences = 0
            with open('inferences.json', 'r') as f: 
                number_inferences = int(json.load(f)["inferences"])
            if number_inferences > 0:
                number_inferences -= 1
                print(number_inferences)
            else: 
                print("None")
            with open('inferences.json', 'w') as fw: 
                fw.seek(0)
                json.dump({"inferences" : str(number_inferences)}, fw) 

            return jsonify({"inferences": str(number_inferences)})
        else:
            return {"error" : "Method not allowed"}
        
    @app.route('/enclave_ready', methods=['GET'])
    def enclave_ready():
        if request.method == "GET":
            return {"state" : "enclave request Ready"}
        else:
            return {"error" : "Method not allowed"}



    return app