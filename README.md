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
Visit http://0.0.0.0:8080 in your web browser to access the API.


**✅ Congratulation ! Your API is now available**

You can test it with [Postman](https://www.postman.com) or on your browser directly.
