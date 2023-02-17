# IoT101 - Smart Bins

This is a smart bins project: [https://es1.harryphillips.co.uk]

## Project Structure

The architecture of the project is as follows: Bins contain a raspberry pi (a **node**), which communicate using gRPC with the **server**, which processes the data received from the nodes and stores it in a database. The server then serves a web app (**frontend**), which requests data from the server via HTTP.

### Directories

* `frontend`: Web app for interfacing with the server
* `server`: Backend server
* `rpi-app`: Application that runs on the nodes
* `marketing`: Marketing website

There are two main software packages: The server docker image, which contains the server, frontend and marketing page, and the Raspberry Pi debian package which contains the node software.

### Getting pre-built packages

The server docker image is pushed to `https://hub.docker.com/r/harry1453/es1-server` and the RPi app package can be downloaded from the assets of the latest `rpi-app/build` GitHub Actions run.

### Building the packages

The server docker image can be built using `docker buildx build .` in the root directory of the repository. This only requires Docker to be installed. Running the image will then start the server. Pass through ports 80 and 81 to access the HTTP and gRPC APIs, respectively.

The raspberry pi image can be built by running `package.sh` from the `rpi-app` directory. This requires several tools, and more documentation including installation instructions is in `rpi-app/README.md`.
