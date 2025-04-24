# Vertrag

Centered type-safety between frontend and backend in multiple languages.

## Table of contents
- [Vertrag](#vertrag)
  - [Table of contents](#table-of-contents)
  - [Remark](#remark)
  - [Why?](#why)
    - [Different requirements](#different-requirements)
    - [But can't we do that before?](#but-cant-we-do-that-before)
  - [Installation](#installation)
    - [OpenAPI Generator](#openapi-generator)
    - [Bun](#bun)
    - [Just](#just)
  - [Usage](#usage)
  - [Features](#features)
  - [What I'm gonna do with this?](#what-im-gonna-do-with-this)
  - [Contributions](#contributions)

## Remark

This repository is more of a proof of concept than a library. 
It contains a set of tools made by amazing people like [oRPC](https://orpc.unnoq.com/) and [OpenAPI Generator](https://openapi-generator.tech/) to help you build a type-safe API between your frontend and backend.
I did not create any of these tools, I just put them together in a way that I think is useful.

So please do me a favor and check out the original projects and give them a star if you like them!

## Why?

### Different requirements

Sometimes your requirement requires that your backend must be in Rust, Go, or Python. Usually, you would
  1. Write your backend
  2. Generate your OpenAPI spec
  3. Use that spec to generate your frontend
That means your frontend kinda has to "wait" for your backend to get the types. 

Now you both can just agree on a spec and implement it in parallel.

### But can't we do that before?

Yes, but writing OpenAPI specs by hand is a pain. 
TypeScript just gives you autocomplete and modularity. 
You can use oRPC as a client (and potentially a mock server as well)

## Installation

Keep in mind that this is not a library, so please install these tools manually.

### OpenAPI Generator

Checkout installation instructions [here](https://openapi-generator.tech/docs/installation).

### Bun

I use `bun` for this project. Check out the [installation instructions](https://bun.sh/docs/installation) if you don't have it yet.

### Just

A script runner at the root of the project. You don't really need it, but it makes it easier to run the scripts in different projects from the root. Check out [Just](https://github.com/casey/just)

## Usage

1. Write oRPC contracts in `playground/contract`
2. Run `bun run gen` to generate the OpenAPI spec
3. Run `just gen-{framework}` to generate the server stubs for the framework of your choice

We have the following frameworks available:
- `axum` (Rust)

## Features

- oRPC's
  - No generation step for the client
  - Potentionally a mock server (but I've not tried it yet)
  - Tanstack Query first-class support
- OpenAPI Generator
  - Server stubs for models and routes (!) in a variety of languages, please check the [OpenAPI Generator](https://openapi-generator.tech/docs/generators) for the full list

## What I'm gonna do with this?

- OpenAPI Generator kinda gives you too little control over the generated code. I don't quite like its naming conventions. Maybe I might build a custom generator for it. 
  - Also it generates the entire server. I just need routes and models.
- Database and Auth.
- Generate oRPC contracts, but this work might be a direct contribution to oRPC instead of a separate project.

## Contributions

I don't have a big vision yet so it's just an experiment for now.
Please contribute directly to the [oRPC](https://github.com/unnoq/orpc/discussions) instead if you have any ideas or suggestions related to it.