# sync.dataroot.ca
sync.dataroot.ca: A Rust service for data synchronization using PostgreSQL and Hexagonal Architecture.

## Table of Contents

<!-- TOC -->

- [sync.dataroot.ca](#sync.dataroot.ca)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Usage](#usage)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Author Information](#author-information)
  <!-- TOC -->

## Overview
The `sync.dataroot.ca` project is a Rust-based service that synchronizes data from various sources to the PostgreSQL database of the `api.dataroot.ca` API. This service is designed following the Hexagonal Architecture principles, ensuring the application is maintainable, scalable, and loosely coupled. Currently, it supports Medium and Meetup as data sources, but is open to future expansion and contributions.


## Usage
The `sync.dataroot.ca` service operates behind the scenes, fetching data from various sources (currently Medium and Meetup) and synchronizing it with the dataroot.ca API's PostgreSQL database periodically. It does not expose an API for direct interaction, instead silently ensures that data is always up-to-date.


## Requirements
To run the `sync.dataroot.ca` backend, you will need the following:

- Rust programming language (version 1.66.1)
- Dependencies specified in the `Cargo.toml` file

## Installation
Use docker-compose to start requirements resources

```bash
docker-compose up -d
```

Create a .env file with this default envs in env.example

```bash
cargo run
```

## Author Information

This module is maintained by the contributors listed on [GitHub](https://github.com/datarootca/sync.dataroot.ca/graphs/contributors).

