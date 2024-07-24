# Simple REST Client

Simple REST Client is an easy-to-use command-line tool for interacting with RESTful APIs. It allows you to make HTTP requests (GET, POST, PUT, DELETE, etc.) and view the responses in a clear and concise format.

**Key Features**

- Simple and intuitive
- Versatile
- Flexible
- Open-source

### Why should use Simple REST Client?

Simple REST Client can help you to be more productive by making it easier and faster to interact with RESTful APIs. This can help you to get your work done more quickly and efficiently. And even more if you are one of those people who love to use the terminal.

## Installation

## Your first RC request

Initialise a configuration file, note that this configuration file is where your base URL will be declared.

```sh
rc --init
```

By default `rc` will create a file in your current folder. If you want to create a global file use the `--global` option when initialising the project.

```jsonc
// rc.config.json
{
    "base": "http://localhost:3000",
    "methods": ["GET", "POST", "DELETE", "PATCH"],
    "mode": "Local"
}
```

For now rc will only support JSON, it is planned that in future versions we will be able to add support for other files such as `.yaml` and `.toml`.

After initialising the project you can use options like `--get` to make a data request.

```sh
rc --get <endpoint>
```

## How do I read the documentation?

> [!NOTE]
> This documentation is in constant work. If you would like to contribute write to me on [X](https://x.com/sebastiandotdev)

Code-related documentation can be found in the [docs](./docs/) folder.

## License

RC is under the **MIT** license.
