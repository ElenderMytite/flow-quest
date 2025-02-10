# The Flow Quest Programming Language. Basic Info

## Compiler or Interpreter

Flow quest is a ***compiler***.

## Typing

It is a strongly typed language(not yet).

## Paradigm
Flow quest has its own paradigm (I don't know did somebody use it before). This paradigm is similar to OOP.

### *I called it neuron paradigm.*
This paradigm implies that the program is a complex neural network. Each neuron in the program can have different difficulty.
Paradigm is similar to object oriented programming but unlike it neuron paradigm is focusing on transfering data between neurons with minimum transformation of it. 
###### (In fact I have not started yet the neuron paradigm)

### Neurons(nodes)
The node(neuron) is a mini program so you can have an entire program in one node(!) or(and) have neurons inside other neurons.
Neuron - list of pointers to routs, storages and transformers

### Routs(streamers and listeners)
Each **neuron** can have any quantity of streamers and listeners (routs). Each route and storage(described below) can be in ownership of multiple nodes.

### Object
###### Can be:
* number
* bool(beta)
* string(coming soon)
* tuple
  * Struct
  * Coming soon
  * transformer takes one object so if you want to pass multiple arguments to it - pass tuple 
* condition
  * enum
  * coming soon
* slice
   * indexed(list)
   * ordered(deque)
   * coming soon
* hashmap
  * dictionary
  * set
  * also coming soon

### Storage
Stores one object

### Transformers
Functions that takes one object and returns another

# Flow Quest is in alpha version!!
## There can be bugs and development of a lot of decribed above hasn't started yet!
