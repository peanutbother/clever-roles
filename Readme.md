<div align="center">
  <p>
    <a href="https://hub.docker.com/r/bricksoft/clever-roles-dc" target="_blank">
      <img alt="Docker Hub Status" src="https://img.shields.io/docker/pulls/bricksoft/clever-roles-dc?logo=docker&style=flat-square"></a>
    <a href="https://github.com/peanutbother/clever-roles/releases/latest" target="_blank">
      <img alt="Github Release" src="https://img.shields.io/github/v/release/peanutbother/clever-roles?logo=github&style=flat-square"></a>
    <a href="https://github.com/peanutbother/clever-roles/stargazers" target="_blank"><img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/peanutbother/clever-roles?logo=github&logoColor=white&style=flat-square"></a>
    <br />
    <a href="https://github.com/peanutbother/clever-roles/actions" target="_blank">
      <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/peanutbother/clever-roles/test.yml?branch=main&label=Tests&logo=github&style=flat-square"></a>
    <a href="https://app.codecov.io/gh/peanutbother/clever-roles" target="_blank">
      <img alt="Codecov branch" src="https://img.shields.io/codecov/c/github/peanutbother/clever-roles/main?logo=codecov&logoColor=white&style=flat-square"></a>
    <a href="https://rust-lang.org/" target="_blank">
      <img alt="rust-edition" src="https://img.shields.io/badge/rust%20edition-2018-blue?logo=rust&style=flat-square"></a>
    <a href="https://docs.rs/poise/latest/poise/" target="_blank">
      <img alt="rust discord library (poise)" src="https://img.shields.io/crates/v/poise?label=poise&logo=discord&logoColor=white&style=flat-square"></a>
    <a href="https://discord.gg/HeaQ7wxDyj" target="_blank">
      <img alt="Discord" src="https://img.shields.io/discord/995301719711957072?logo=discord&logoColor=white&style=flat-square"></a>
  </p>
</div>

# Clever Roles Discord Bot

## About

This discord bot adds / removes guild members role(s) when they join or leave a voice channel.
You can configure which channel triggers which role(s).

This allows you to create channels which are only visible while being in a stream voice chat, for extended usage if voice channel chats aren't enough.

## Setup

To get started invite the bot and run the setup with `/watch` to watch a voice channel and select the matching role to manage.
You can `/unwatch` this channel / role later.

To make this bot active you also need to `/activate` it in order for it to actually watch the channels you set up.
You can `/deactivate` the bot later if you pause it, or kick it if you no longer wish to use it.

## Development / Deployment

This bot is running inside docker and assumes certain environment variables to be set to function properly.

- TZ: defaults to `Etc/Utc`, a timezone to create proper timestamps in logs and some discord api
- DATABASE_URL: defaults to `sqlite:/data/database.sqlite`, a `seaorm sqlite` compatible file url
- RUST_LOG: optinal - overrided to `error,clever_roles=info` in production, allows you to set more verbose logging if needed
- DISCORD_TOKEN: required, your discord api token

To deploy the container either pull `bricksoft/clever-roles-dc` or build it locally by `docker build .` and then run it:
> docker run -d -v PATH_TO_YOUR_DATA:/data -e DISCORD_TOKEN=YOUR_TOKEN_HERE bricksoft/clever-roles-dc:latest
