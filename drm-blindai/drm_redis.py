from flask import Flask, session, redirect, url_for, request, jsonify
from flask_session import Session
import os
import redis
import json
from dotenv import load_dotenv


load_dotenv()

class Config:
    """Flask configuration for redis"""
    SECRET_KEY = os.getenv('SECRET_KEY')
    FLASK_APP = os.getenv('FLASK_APP')
    FLASK_ENV = os.getenv('FLASK_ENV')

    # Flask-Session
    SESSION_TYPE = os.getenv('SESSION_TYPE')
    SESSION_REDIS_ENV = os.getenv('SESSION_REDIS')
    SESSION_REDIS = redis.from_url(SESSION_REDIS_ENV)

sess = Session()



# DRM server, goals: 
#   - listening requests from the Inference Server for comsuption budget
#   - sending each inference for tracing
def drm_server():
    app = Flask(__name__)

    @app.route('/request_consumption', methods=['POST', 'GET'])
    def request_consumption():
        if request.method == 'POST':
            # we can imagine that it looks into a database to see if the number of inferences asked for 
            # is allowed
            number_inferences = request.form['number_inferences']
            print("* [POST: /request_consumption] New number of inferences : " + str(number_inferences))

            with open('inferences.json', 'w') as f: 
                json.dump({"inferences" : str(number_inferences)}, f)
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