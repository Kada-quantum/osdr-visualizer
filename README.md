# OSDR Visualizer
OSDR Visualizer is an AI web application tool that generates flowchart for any Open Science Data.
It uses NASA API for Open Science Data Repository to retrieve the description which is then summerized by Llama3.2 and the summery is visually represented into a flowchart.
# Demo
[![Demo]((https://github.com/Kada-quantum/osdr-visualizer/raw/refs/heads/main/assets/example2.mp4))](https://github.com/Kada-quantum/osdr-visualizer/raw/refs/heads/main/assets/example2.mp4)
# Usage
 - Install Pixi
 - Install Ollama
 - Clone:
   ```
   $ git clone https://github.com/Kada-quantum/osdr-visualizer.git
   $ cd osdr-visualizer
   ```
 - Set up project environment:
   ```
   $ pixi install
   ```
 - Run Ollama server (If not ran already):
   ```
   $ ollama serve &
   ```
 - Run the web application:
   ```
   $ pixi run python app.py
   ```
### Options
```
$ pixi run python app.py --help
usage: app.py [-h] [-m MODEL] [-a ADDRESS] [-p PORT]
options:
  -h, --help            show this help message and exit
  -m MODEL, --model MODEL
                        Set ollama model [default: llama3.2]
  -a ADDRESS, --address ADDRESS
                        Set ollama server address [default: localhost]
  -p PORT, --port PORT  Set ollama server port [default: 11434]
```
