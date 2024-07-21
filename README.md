# Simple REST Client

Simple REST Client is an easy-to-use command-line tool for interacting with RESTful APIs. It allows you to make HTTP requests (GET, POST, PUT, DELETE, etc.) and view the responses in a clear and concise format.

**Key Features**

- Simple and intuitive
- Versatile
- Flexible
- Open-source

### Why should use Simple REST Client?

Simple REST Client can help you to be more productive by making it easier and faster to interact with RESTful APIs. This can help you to get your work done more quickly and efficiently. And even more if you are one of those people who love to use the terminal.

## Usage

The use of RC is very easy. we are working to make it intuitive with great development experience

### Initialize a file configuration for RC

1. It will create a file globally.

```zsh
rc init --config --global
```

2. It will create a file locally.

```zsh
rc init --config --local
```

### Config file

Actually only we have support to JSON

```jsonc
// rc.json

{
  "baseURL": "http://localhost:3000",
  "methods": ["GET", "POST", "PATCH", "DELETE"]
}
```

## How do I read the documentation?

Code-related documentation can be found in the [docs](./docs/) folder.

## License

RC is under the **MIT** license.
