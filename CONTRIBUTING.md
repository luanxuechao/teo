# Welcome!

Welcome! You've arrived at our Contributing page and are now one step away from joining our quest to make coding more 
fun. We're thankful for all your contributions, whether it's helping us find issues in our code, highlighting features 
we're missing, or contributing to the codebase. If you've found your way here, you'll soon be ready to join in the fun 
of building features and fixing bugs directly with us - and we're thrilled to have you on board!

To get you started on a good foot, we've created an easy overview of the most important things to get you started 
contributing code to Teo below as well as a 
[Code of Conduct](https://github.com/teocloud/teo/blob/master/CODE_OF_CONDUCT.md) for contributing to the development 
of Teo.

## Contributing Code

Welcome to the monorepo for the TEO HTTP server framework.

## General Prerequisites

* Core: Rust `>=1.67`, latest is recommended
* Clients:
  * TypeScript & JavaScript: Node.js `>=17`, latest or lastest LTS is recommended
* Servers:
  * Node.js: Node.js `>=17`, latest or lastest LTS is recommended
  * Python: Python `>=3.11`, latest is recommended
  * Go: Go `>=1.20`, latest is recommended

## Building Teo when you make changes

* Core: `cargo run`

## Tests

Unit tests will be set up later on.

## Conventions

### Code Commit

Please do not write code in languages other than English.

### Git Commit Messages

We structure our messages like this:

```
<type>(<feature>): <subject>
<BLANK LINE>
<body>
```

#### List of types:

* feat: A new feature
* fix: A bug fix
* docs: Documentation only changes
* style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
* refactor: A code change that neither fixes a bug nor adds a feature
* perf: A code change that improves performance
* test: Adding missing or correcting existing tests
* chore: Changes to the build process or auxiliary tools and libraries such as documentation generation

#### List of features in the monorepo:

* sql: The SQL connector
* mongodb: The MongoDB connector
* server: The server integration
* core: The core
* pipeline: The pipeline items
* lang: The schema language
* client: Client generators
* entity: Entity generators

## Legal

Pull Request authors must sign the [TEO Cloud CLA](https://cla-assistant.io/teocloud/teo), it will show up in an 
automated comment after you create a PR.

If you cannot or do not want to sign this CLA (e.g. your employment contract for your employer may not allow this), you 
should not submit a PR. Open an issue and someone else can do the work.
