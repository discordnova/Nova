# Nova Internals - Management

In order to run in multiple environments, Nova uses a scheduling system to balance shards between "cluster" instances.
Each cluster instance is in fact a gateway instance. All the management is handled in the "manager" component. 
The discord shards are dynamically scaled and restarted if needed.