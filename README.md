
# GDSetup

## Motivation

I recently tried out the 'new' GDExtension for Godot 4 and I noticed that creating and managing Godot projects which use a GDExtension module felt very tedious. So I wanted to automate that process.
This **Command-line interface tool** aims to provide functionality based on that problem.

## Features

- (Kind of done) Create a new Godot-GDExtension via a single command (`gdsetup`)
- (WIP) Modify existing modules to change the classname and/ or module name (`gdsetup rename NewName`)
- (WIP) Add new modules to an existing project

## Getting started

As this tool is written using Rust, there are two ways to get it running right now.

### Option 1: Getting the precompiled binary

Download and call the executable via command line.

### Option 2: Compiling it yourself

Install Rust and run `cargo build --release`