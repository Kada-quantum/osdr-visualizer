# OSDR Visualizer
OSDR Visualizer is an AI web application tool that generates flowchart for any [Open Science Data](https://www.nasa.gov/osdr/).
It uses [NASA API](https://api.nasa.gov/) for Open Science Data Repository to retrieve the description which is then summerized by [Llama3.2](https://ollama.com/library/llama3.2) and the summery is visually represented into a flowchart.
Flowchart is generated by [flowchart.js](https://flowchart.js.org/) and the web server is hosted using [flask](https://github.com/pallets/flask).
# Demo
https://github.com/user-attachments/assets/45f5ffee-354b-4028-ac32-d84b7dd95b8e
# Usage
 - Install [Pixi](https://prefix.dev/)
 - Install [Ollama](https://ollama.com/)
 - Download llama3.2:
   ```
   $ ollama pull llama3.2
   ```
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
# License
The software is licensed under the MIT license http://opensource.org/licenses/MIT. This file may not be copied, modified, or distributed except according to those terms.

The image assets were AI generated by Canva Magic Design™ and has no restriction on use in public domain.
