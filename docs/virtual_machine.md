# Reishi Programming Language Virutal Machine (official name tbd)

#### Author: Liam Eckert

### Virtual Machine Specifications

The virtual machine will be stack based. It is easier to implement and I don't think there will be much loss in performance. This should also simplify the instruction set. Right now I am thinking of implementing a sweeping garbage collector similar to what Julia uses, the language should produce very little garbage but there still needs to be a garbage collector.

## Choosing Stack Over Register Based:

Deciding between a stack based and register based virtual machine was difficult,
but choosing stack based came down to a few things.

* Easier to implement
* Toy project
* Simplicity of instructions

