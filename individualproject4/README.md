# A Rust Keywordsr Service with OpenAI API 

Translates difficult text into simpler concepts.

The service has been deployed to duke virtual machine. Access it from [here](http://vcm-30756.vm.duke.edu:8080/)!


- Set it as the value of "OPENAI_API_KEY" in your local environment as an exported variable. e.g.,
```bash
export OPENAI_API_KEY="xxx"
```

- Run the program
```bash
cargo run
```
- Launch http://127.0.0.1:8080 in the browser"


. AWS App Runner
Containerize: follow the instructions from here.
Pay attention to the port in the program and in the setting of the App Runner
Build container out of the Docker image: run make build --build-arg OPENAI_API_KEY="xxx"
Clean build: docker build --no-cache --build-arg OPENAI_API_KEY="xxx" -t summarize . or run make build-no-cache --build-arg OPENAI_API_KEY="xxx"
