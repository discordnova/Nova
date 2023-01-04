# Nova

[![Build](https://github.com/discordnova/Nova/actions/workflows/build.yml/badge.svg)](https://github.com/discordnova/Nova/actions/workflows/build.yml)

## What is nova ?

Nova is a complete framework for building reliable and stable discord bots
using a services based system to operate all the components needed to operate
a discord such as the discord gateway, new discord webhooks for receiving interactions.
Using a traditional infrastructure (sharder / process), we can quickly reach bottlenecks
when we scale to multiple thousands of shards. Nova is a way to avoid these problems by
building a scale-first discord framework that creates an environment that allow 
better testing, reliability and operations easier.

### Advantages

With the help of Nova, you can achieve a number of things, such as

* Scaling of workers independent of the number of shards
* zero-login updates
* Automatic shards scaling
* Shared cache for the whole bot (without broadcastEval and other unsafe methods)
* Stateless workers, easier to test
* Distributed rest rate-limiting
* Easier fine-tuned monitoring using cloud-native technologies
* Languages agnostic apis

### How did we solve this ?

Nova separates the gateway into multiple smaller components corresponding to each
discord apis



[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fdiscordnova%2Fnova.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fdiscordnova%2Fnova?ref=badge_large)
