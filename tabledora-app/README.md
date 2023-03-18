# Tabledora Application

This folder contains the source code for the Tabledora desktop application.

See the folder `/tabledora-website` for the source code for the Tabledora website (tabledora.com).

## Developer Guide

### Setting up for local development

**1. Install Rust**

Visit the [official Rust website](https://www.rust-lang.org/learn/get-started) for instructions to install Rust.

**2. Install NodeJS**

We use `asdf` for managing Node versions. The file `.tool-versions` specifies which version `asdf` should use.

I recommend that you install `asdf` and then use `asdf` to install the right NodeJS version.

**3. Install NodeJS dependencies**

Run the following in your terminal:

```console
npm install 
```

**4. Build and run the desktop application.**

Run the following in your terminal:

```console
npm run tauri dev
```
