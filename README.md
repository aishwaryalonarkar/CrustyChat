This is a Chat application powered by Rust Programming Language which focuses on the Database Integrity with Replication & Hashing.
This project consists of 4 main components. 
    1. Rust Server
    2. Client
    3. ActiveMQ 
    4. Redis Database

Installation :
1. Clone the repository.

2. Installation for AngularJS Client
    1. Open a terminal or command prompt.
    2. Install Node.js and npm (Node Package Manager) on your system if they are not already installed. You can download them from https://nodejs.org/en/download/.
    3. Run the following command to install AngularJS globally on your system:
        > npm install -g @angular/cli@13.3.0
    Note: This will install the latest version of AngularJS available on npm.
    Once the installation is complete, you can verify it by running the following command in the terminal or command prompt.
        > ng --version
    This should display the version of AngularJS installed on your system.
    Note: If you encounter any issues during installation or with running AngularJS, you can refer to the official documentation at https://docs.angularjs.org/guide or seek help from the community.
    4. Once AngularJS ins installed navigate to the folder Frontend
        > cd Frontend
    5. Install node modules
        > npm install
    6. Start the Client
        > ng serve

2. Installation for Rust Server
    1. Go to the official Rust website at https://www.rust-lang.org/tools/install
    2. Click on the "Install Rust" button.
    3. Follow the instructions for your specific operating system.
    4. Once Rust is installed, navigate to the server
        > cd database_storing_service
    5. Start the server
        > cargo run

3. Installation of ActiveMQ
    1. Go to the official ActiveMQ website (http://activemq.apache.org/) and navigate to the "Download" page.
    2. Download the binary distribution package for your operating system and extract it to a directory of your choice.
    3. Set the ACTIVEMQ_HOME environment variable to point to the directory where you extracted the package.
    4. (Optional) Change the default uri in apache-activemq-5.17.4/conf/activemq.xml from 0.0.0.0 to your ip address or 127.0.0.1 
    Note : Based on the ip set here, configure the ACTIVEMQ_IP in setup/app.py to the same ip address.
    4. Start the ActiveMQ server by running the activemq command (or activemq.bat on Windows) from the bin directory.
        > cd apache-activemq-5.17.4\bin
        > activemq start

4. Installion of Python
    1. Go to the official Python website (https://www.python.org/downloads/) and download the latest version of Python for your operating system.
    2. Run the installer and follow the prompts to install Python on your system.
    3. Once the installation is complete, you can verify that Python is installed correctly by opening a command prompt or terminal window and typing python --version. This should display the version number of Python that you just installed.
    4. (Optional) Once python is installed create a virtual environment
        > pip install virtualenv
        > virtualenv pyenv
        > source venv/bin/activate
        > venv\Scripts\activate
    5. Install the requirements
        > pip install -r requirements.txt
    6. Start the server 
        > python app.py

5. Installation of Redis 
    1. First, make sure you have a C compiler installed on your system, as Redis is written in C. If you're on a Debian-based system, you can install the build-essential package by running:
        > sudo apt-get update
        > sudo apt-get install build-essential
    2. Next, download the latest stable version of Redis from the official website using the following command:
        > wget http://download.redis.io/redis-stable.tar.gz
    3. Extract the downloaded archive using the following command:
        > tar xzf redis-stable.tar.gz
    4. Change to the extracted Redis directory:
        > cd redis-stable
    5. Build Redis by running:
        > make
    6. After the build process is complete, you can install Redis by running:
        > make install
    7. By default, Redis is installed in /usr/local/bin. To confirm that Redis has been installed successfully, you can run the following command:
        > redis-server --version
        This should output the version of Redis installed on your system.
    8. Since this application saves in json format and by default json module is not installed with redis installation do the following.
        > git clone https://github.com/RedisJSON/RedisJSON.git
        > cd RedisJSON
        > cargo build --release
        > cd target/release/
        > mkdir /etc/redis/modules/
        > mv librejson.so /etc/redis/modules/
    9. Configure the Redis server to run on multiple ports.
        open redis.conf
        add the following lines below port 6379.
        port 6380
        port 6381
    10. Start the Redis Servers.
        > redis-server --port 6379 --loadmodule /etc/redis/modules/rejson.so
        > redis-server --port 6380 --loadmodule /etc/redis/modules/rejson.so
        > redis-server --port 6381 --loadmodule /etc/redis/modules/rejson.so

Set replication by opening the link http://127.0.0.1:8081/set_replication 
<!-- This is the rust api which sets replication of master server 6379 to slave server 6380-->

Finally to use the application on the browser open link http://localhost:4200/














