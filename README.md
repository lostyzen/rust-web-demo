# 🦀 Rust Web Demo

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/tokio-rs/axum)

📖 [English version](#-rust-web-demo-1)

Une démonstration simple de page web en Rust avec Axum. Montre comment séparer les templates HTML du code Rust pour une meilleure organisation.

## ✨ Fonctionnalités

- 🚀 Serveur web asynchrone avec Axum
- 🎨 Templates HTML séparés du code Rust
- 🎯 Code propre et maintenable

## 🛠️ Installation

Assurez-vous d'avoir Rust installé sur votre système :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clonez le repository :

```bash
git clone https://github.com/lostyzen/rust-web-demo.git
cd rust-web-demo
```

Installez les dépendances :

```bash
cargo build
```

## 🚀 Utilisation

Lancez le serveur :

```bash
cargo run
```

Le serveur sera accessible sur `http://127.0.0.1:3000`

### Page disponible

- `GET /` - Page d'accueil avec template HTML

## 📁 Structure du projet

```
rust-web-demo/
├── Cargo.toml          # Configuration du projet Rust
├── src/
│   ├── main.rs         # Point d'entrée de l'application
│   └── templates/      # Templates HTML
│       └── home.html   # Template de la page d'accueil
└── README.md           # Ce fichier
```

## 🏗️ Technologies utilisées

- **Rust** - Langage de programmation système
- **Axum** - Framework web asynchrone
- **Tokio** - Runtime asynchrone
- **Tower** - Bibliothèque de services HTTP

## 📄 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

---

# 🦀 Rust Web Demo

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/tokio-rs/axum)

A simple web page demo built with Rust using the Axum framework. Demonstrates how to separate HTML templates from Rust code for better organization.

## ✨ Features

- 🚀 Asynchronous web server with Axum
- 🎨 HTML templates separated from Rust code
- 🎯 Clean and maintainable code

## 🛠️ Installation

Make sure you have Rust installed on your system:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository:

```bash
git clone https://github.com/lostyzen/rust-web-demo.git
cd rust-web-demo
```

Install dependencies:

```bash
cargo build
```

## 🚀 Usage

Start the server:

```bash
cargo run
```

The server will be available at `http://127.0.0.1:3000`

### Available page

- `GET /` - Home page with HTML template

## 📁 Project structure

```
rust-web-demo/
├── Cargo.toml          # Rust project configuration
├── src/
│   ├── main.rs         # Application entry point
│   └── templates/      # HTML templates
│       └── home.html   # Home page template
└── README.md           # This file
```

## 🏗️ Technologies used

- **Rust** - Systems programming language
- **Axum** - Asynchronous web framework
- **Tokio** - Asynchronous runtime
- **Tower** - HTTP services library

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.