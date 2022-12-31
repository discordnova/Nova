---
sidebar_position: 2
---

# 5 Minutes quickstart

This page shows you how to start a new nova project
under five minutes using a typescript project,
hold tight this is going to be fast.

## Requirements

* A discord bot application available
* [Docker](https://docker.io/) (or alternatives) available to you.
* A domain name / [ngrok.io](https://ngrok.com/) domain (for webhooks only)

> If you are deploying nova to production, consider following the
> production guide instead.

## Setting up a nova instance

Clone the [example repo](https://github.com/discordnova/nova-quickstart.git) like so

`git clone https://github.com/discordnova/nova-quickstart.git`,

In this folder, find the `config.yml.example` file and rename it to match `config.yml`

Next, you need to fill all the environment variables to match your discord bot.
