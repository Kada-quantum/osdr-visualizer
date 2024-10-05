from ollama import Client
client = Client(host='http://localhost:11434')

stream = client.generate(
                           model='llama3.2:3b-instruct-fp16',
                           prompt="outline the experiment",
                           system="the experiment is to calculate 1+1 in the head ten thousand times and see if an error is present.",
                           stream=True
                         )
for chunk in stream:
    print(chunk['response'], end='', flush=True)
    if chunk['done']:
        break
