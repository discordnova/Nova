# Workspace/System structure

## System structure

In the nova repository, two different types of projects exist, 

* The management projects primarly in Go \
    They manage the other components of the nova infrastructure.
* The data-path projects \
    They handle all the data transfer / management.

### Gateway

> The gateway interfaces with the discord gateway to retrive events in real time
It's implemented in rust and is in the gateway folder.