
# Minimal Rust Backend framework

This is just me trying to make a minimal Backend framework for the Rust programming language, which I chose as my first Rust project to challenge myself.

## Badges

[![GPLv3 License](https://img.shields.io/github/license/Nerscylla/minimal-backend-framework.rs?style=for-the-badge&logo=GNU&color=red&logoColor=white)](https://opensource.org/license/gpl-3-0)

[![Rust Language](https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=Rust&logoColor=white)](https://www.rust-lang.org/)

![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FNerscylla%2Fminimal-backend-framework.rs%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=version&logo=V&logoColor=white)

![Build](https://img.shields.io/github/actions/workflow/status/Nerscylla/minimal-backend-framework.rs/compile.yml?event=push&style=for-the-badge&logo=zsh&logoColor=white&label=build)

![Tests](https://img.shields.io/github/actions/workflow/status/Nerscylla/minimal-backend-framework.rs/test.yml?event=push&style=for-the-badge&logo=githubactions&logoColor=white&label=tests)
## Roadmap

- [ ]  abstract response into a seperate Struct
    - [ ]  status code
    - [ ]  headers
    - [ ]  Body content / content type
- [ ]  more proper error Handling
    - [ ]  400 bad request; 404's; 500's
    - [ ]  customizable error pages
- [ ]  requests
    - [ ]  query strings
    - [ ]  form parsing
- [ ]  testing
    - [ ]  parsing / routing logic
- [ ]  middleware support
    - [ ]  pre-handler
    - [ ]  post-handler
- [ ]  static files (possibly as middleware)
## Features

- Adding Routes
- Returning Strings
- getting query path
- getting raw request body
- getting request headers and method


## Support

In case you have any support requests, shoot me a message on [Discord (@notacrazycatgirl)](https://discord.com/users/799599681280802848) or email me at the.baum124345@gmail.com.


## Feedback

If you have any feedback, please reach out to me at the.baum12345@gmail.com or over [Discord (@notacrazycatgirl)](https://discord.com/users/799599681280802848).

## Authors

- [@Nerscylla](https://github.com/Nerscylla)