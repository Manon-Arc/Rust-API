# ⚙️ Rust API :

Project by  __ARCAS__ Manon

This project is aimed at providing a web API in Rust language.

This README will guide you through setting up your environment, installing the required packages, and starting the project.


### 📌 Table of contents :

I. [Badges](#🎯-badges)

II. [Prerequisites](#🔧-prerequisites)

III. [Availables Features](#💡-availables-features)

IV. [Endpoints](#​📋​-endpoints)
  1. [Ping](#1-Ping)
  2. [Others](#2-Others)

V. [Installing the project](#💻-project-installing)

VI. [Starting the project](#💻-project-start)


### 🎯 Badges :

[![Rust badge](https://img.shields.io/badge/Language-Rust-black
)](https://www.rust-lang.org/fr)
[![React badge](https://img.shields.io/badge/Library-actix_web-e26af2)
](https://fr.react.dev)
 ![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=flat&logo=docker&logoColor=white)


### 🔧 Prerequisites :
- [Rust](https://www.rust-lang.org/fr) installed on your system.


### 💡 Availables Features :
This solution deserve a web API in Rust language with some entrypoints detailled below.

### ​📋 Endpoints :

#### 1. Ping :
The entrypoint `/ping` will send the hearders of your request in json format.

#### 2. Others :
All of the others routes will resend a status code 404 with a blank content.

### 💻 Project installing :

#### 1. Clone the Repository
```bash
git clone https://github.com/Manon-Arc/Rust-API.git
```

#### 2. Configure environment variable  :
You have to configure a variable environment `PING_LISTEN_PORT` on your computer. Otherwise, the `8080` port will be used by default.

#### 3. Navigate to the Project Directory
```bash
cd Rust-API
```

### 💻 Project start :

#### 1. Start the Development Server :
```rs
cargo run
```

#### 2. Access the API :
Visit http://localhost:8080 in your web browser to access the API.

## 💻 Use Docker for the project :

### 1. Clone the Repository :
```bash
git clone https://github.com/Manon-Arc/Rust-API.git
```

### 2. Navigate to the Project Directory :
```bash
cd Expresse-API
```

### 3. Build the Docker Image :

#### 1. Dockerfile Simple-stage :
You can select [this Dockerfile](./Dockerfile) for a image in simple a stage version.


```bash
docker build -t api-rust:latest .
```

#### 2. Dockerfile Multi-stage :
You can select [this Dockerfile](./Dockerfile2) for a image in a multi stage version.

```bash
docker build -t api-rust2:latest -f .\Dockerfile2 .
```

### 4 . Run the container : 

```bash
docker run -p 8080:8080 api-rust2
```

### 5. Scan the Dockerfile : 

```bash
trivy image api-rust | Out-File -FilePath resultats_scan.txt -Encoding utf8
```
You can vizualise the result of the scan in this files :
- [Simple stage version Dockerfile](./resultats_scan.txt)
- [Multi stage version Dockerfile2](./resultats_scan2.txt)


**✅ Congratulation ! Your API is now available**

You can test it with [Postman](https://www.postman.com) or on your browser directly.
