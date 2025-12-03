# What's the Point?

The point is a pair of integer co-ordinates on a cartesian plane, each represented using the i32 type.

## Ok, but what would I use it for?

It is intended to be used to represent a location in two dimensional space, but also has support for vector operations
such as scalar multiplication and the dot product.

## Using the library

To use the library as part of one's rust project, simply add the following line to the dependencies section of your 
Cargo.toml file:

>   point = { git = "https://github.com/That-H/point", tag = 0.6.1 }

Note that this will require the use of Cargo to build the project.
The tags used for commits are intended to follow [semver](https://semver.org).

## Viewing documentation

Doing this will require a local copy of the source files, which can be acquired via running the following
command in a new terminal or command prompt window:

>   git clone https://github.com/That-H/point

Then simply run `cargo doc --open --no-deps` to open the docs in a new browser window. 
