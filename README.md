
# rust-web
This is a web project using Rust, Actix-web, Yugabyte, Diesel


## Steps to Use the Project

1. Install the docker into your machine from this site [https://docs.docker.com/engine/install/](https://docs.docker.com/engine/install/)
2. Pull the Yugabyte container using docker by this command: ```sudo docker pull yugabytedb/yugabyte```
3. Run the Yugabyte docker container using this command: \
   ```sudo docker run -d --name yugabyte -p7000:7000 -p9000:9000 -p5433:5433 -p9042:9042 -v ~/yb_data:/home/yugabyte/yb_data yugabytedb/yugabyte:latest bin/yugabyted start --base_dir=/home/yugabyte/yb_data --daemon=false```
4. Ensure that the image has been run by this command ```sudo docker ps -a```, you will find the image name, container id and some other options
5. Open the terminal in the project path and type this command: ```cd yugabyte```
6. Run this command ```diesel setup``` to create the database in the .env file.
7. Install the cargo-swagger into the project and use the extracted yaml file into this site [https://editor.swagger.io/](https://editor.swagger.io/) to see all endpoints with example, and the model in more details.
8. Run the Server from the main file and try to use the endpoints from the swagger site.
