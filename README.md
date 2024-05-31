# Inverse Kinematics
This repo contains use of inverse kinematics to move a leg
Algorithm used is from [this paper](http://www.andreasaristidou.com/publications/papers/IK_survey.pdf) in chapter 4.

# Compiling and running
You need rust and cargo to compile and run the project

> ## Windows additional instructions
> If you don't have clang, install it otherwise raylib won't compile. <br>
> I downloaded [this one](https://github.com/llvm/llvm-project/releases/tag/llvmorg-18.1.5) and it worked out of the box.

- Clone the repo

`git clone https://github.com/xMelisek/IK`

- `cd` into the repo

`cd IK`

- Run the project

`cargo run`