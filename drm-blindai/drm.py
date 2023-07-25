from flask import Flask



# DRM server, goals: 
#   - listening requests from the Inference Server for comsuption budget
#   - sending each inference for tracing
def drm_server():
    app = Flask(__name__)
    @app.route('/hello')
    def hello():
        return 'Hello, World!'
    
    return app