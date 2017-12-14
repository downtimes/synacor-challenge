# My solution for the Synacor Challange

This repository contains all my files and electronically written thoughts
I produced while trying to solve the [Synacor Challenge](https://challenge.synacor.com/) 

I can recommend to anyone to try their hand on this fabulous
challenge! (Best to go in spoiler free. So stop reading here!)

## How to run it

My solution is written in rust so a simple
```
cargo run --bin synacor-challenge
```
In the root folder should let you experience the WM running with the 
provided binary blob.

If you want to see the output of playing the whole game to the end, you can
use the provided play\_game.sh script, or look for the correct moves in this
file. The file contains a playtrough of the game realized with expect/tcl.

The helper programs can all be launched with cargo too
```
cargo run --bin coin-solver
cargo run --bin r7
cargo run --bin maze
```

function.cpp was a transcript of the r7 check algorithm I wrote while trying
to work through the dissasembly.

The two .txt files are transcripts of items you will find inside the game.

dissasembly is the whole binary file run through the decode and fetch state
of the WM starting from address 0 up to the end. There are some minor comments
in there too. The OUT instruction also has the character it would print behind
it if the operand was a constant.

arch-spec and challenge.bin are the inputs provided by the Synacor-Challenge
homepage.
