from flask import Flask, jsonify, make_response, request, render_template
import os
import time
import requests

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')


@app.route('/upload', methods=['POST'])
def upload():
    
    # Get the file from the form
    file = request.files['image']

    # Save the file to the backend
    file_path = os.path.join('image', file.filename)
    file.save(file_path)

    style = request.form.get('style')
    headers = {
        'Accept': 'application/json',
    }
    
    data = {
        'api_key': 'VCGKSUTDMUCYMOHTEPAYYBGGKTCWBXCQGCTNGPIADSKRSRYJ',
        'style_id': style,
    }

    files = {
        'photo' : open(file_path, 'rb')
    }
  
    response = requests.post('https://neuralstyle.art/api.json', headers=headers, data=data, files=files)
    
    
    return response.text

@app.route('/progress/<filterjob_id>', methods=['POST'])
def get_progress(filterjob_id):

    params = {
        'api_key': 'VCGKSUTDMUCYMOHTEPAYYBGGKTCWBXCQGCTNGPIADSKRSRYJ',
    }

    response = requests.get('https://neuralstyle.art/api/' + filterjob_id + '.json', params=params)

    
    return response.text



    

if __name__ == '__main__':
    app.run(host='0.0.0.0', port='8080')