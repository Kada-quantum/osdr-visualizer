import argparse
from flask import Flask, render_template, request
import requests
import json
from ollama import Client

app = Flask(__name__)
# model='llama3.2'
model='llama3.2:3b-instruct-fp16'

# Set up the Ollama call as a function
def call_ollama_api(description):
    global model
    client = Client(host='http://localhost:11434')

    stream = client.generate(
                               model,
                               prompt="Generate a step-by-step process of the experiment in numerical order:\n\nSteps:",
                               system=description,
                               stream=True
                             )
    result = ""
    for chunk in stream:
        print(chunk['response'], end="", flush=True)
        result += chunk['response']
        if chunk['done']:
            break
    return result

def fetch_osd(osd_number):
    resp = requests.get(f'https://osdr.nasa.gov/osdr/data/osd/meta/{osd_number}')
    data = resp.json()
    return data['study'][f'OSD-{osd_number}']['studies'][0]['description']

# Define the home route to take OSD input
@app.route('/')
def index():
    return render_template('index.html')

@app.get('/OSD/<int:osd_id>')
def converter_with_type(osd_id):
    osd_number = osd_id
    description = fetch_osd(osd_number)

    
    # Call the Ollama API to generate steps
    steps_response = call_ollama_api(description)
    response = make_response(steps_response, 200)
    response.mimetype = "text/plain"
    return response

# Define the result route that will handle form submission and display the results
@app.route('/result', methods=['POST'])
def result():
    osd_number = request.form['osd_number']
    description = fetch_osd(osd_number)
    
    
    # Call the Ollama API to generate steps
    steps_response = call_ollama_api(description)
    print(steps_response)
    
    # Extract steps and format them
    final_list = []
    if steps_response:
        formatted_steps = steps_response.splitlines()
        step_index = 1
        for line in formatted_steps:
            line = line.strip()
            if line and (line[0].isdigit() or line.startswith("Note:")):
                if line.startswith("Note:"):
                    final_list.append(line)
                else:
                    cleaned_line = ' '.join(line.split()[1:])
                    final_list.append(f"{step_index}. {cleaned_line}")
                    step_index += 1
    else:
        final_list.append("No steps were generated.")
    
    return render_template('result.html', steps=final_list)

# Run the Flask app
if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("-m", "--model", help = "Set ollama model [default: llama3.2]")
    args = parser.parse_args()
    if args.model:
        model = args.model
    app.run(debug=True)
