
# GDSetup

## Motivation

I recently tried out the 'new' GDExtension for Godot 4 and I noticed that creating and managing Godot projects which use a GDExtension module felt very tedious. So I wanted to automate that process.
This **Command-line interface tool** aims to provide functionality based on that problem.

## Features

- Create a new Godot-GDExtension via a single command (`gdsetup init projectname`)
- (Usable but not done) Modify existing modules to change the classname and/ or module name (`gdsetup rename oldname newname -p path/to/project`)
- (WIP) Add new modules to an existing project

## Getting started

As this tool is written using Rust, there are two ways to get it running right now.

### Option 1: Getting the precompiled binary (Not possible yet)

(Download and call the executable via command line.)

### Option 2: Compiling it yourself

Install Rust and run `cargo build --release`

### Setting it up

For ease of use, I recommend adding the path to the compiled binary to your path.