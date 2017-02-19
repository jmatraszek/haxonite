# ![Haxonite](src/img/haxonite.png)

# Haxonite: Rock-solid API mocking for hackers

Haxonite is an easy-to-use API mocking server written in Rust.

## Table of Contents

   * [Haxonite: Rock-solid API mocking for hackers](README.md#haxonite-rock-solid-api-mocking-for-hackers)
      * [Installation](#installation)
      * [Usage](#usage)
      * [Example](#example)
      * [Disclaimer](#disclaimer)
      * [Changelog](#changelog)
      * [Contributing](#contributing)
      * [Questions and issues](#questions-and-issues)
      * [Self-promotion](#self-promotion)
      * [License](#license)

## Installation

You can install Haxonite by running `cargo install haxonite`. Alternatively,
you can clone the repository and compile (or download) the project
yourself by running `cargo build --release`.

<sup>If you do not have Rust already installed, you can install it
following [these
instructions](https://www.rust-lang.org/en-US/install.html).</sup>

## Usage

```
$ haxonite --help
Haxonite 0.1.0 Jakub Matraszek
<jakub.matraszek@gmail.com> Easy API mocking

USAGE:
    haxonite [OPTIONS] [SUBCOMMAND]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file. Default: config.toml.
    -h, --host <HOST>      Run Haxonite on the host. Default: localhost.
    -p, --port <PORT>      Run Haxonite on the specified port. Default: 4000.

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    new     Create new Haxonite project
```

```
$ haxonite new --help
haxonite-new 0.1.0
Jakub Matraszek <jakub.matraszek@gmail.com>
Create new Haxonite project

USAGE:
    haxonite new [FLAGS] <project_name>

FLAGS:
    -f, --full       Use to create a project with full config.toml file
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <project_name>    Name of the project (will be used as a directory name)
```

**You can find out more on [using Haxonite](USAGE.md).**

## Example

Create a new project (you can also use `--full` option to generate full-blown
config example):

```
$ haxonite new example
19:25:09 [INFO] haxonite::utils: Creating new project: example!
```

Following `config.toml` config file was created...

```
[server]
port = 4000
host = "localhost"

[requests.example_request]
method = "GET"
path = "/"
[[requests.example_request.responses]]
status = 200
headers = [
	"Content-Type: application/json"
]
response = "responses/haxonite.json"

[requests.assets_request]
type = "static"
path = "/public"
[[requests.assets_request.responses]]
response = "assets"
```

...along with an example JSON response:

```
$ cat responses/haxonite.json
{
  "haxonite": {
    "version": "{{ haxonite_version }}",
    "authors": "{{{ haxonite_authors }}}",
    "logo": "assets/haxonite.png"
  }
}
```

Change directory to a newly generated project's directory and start
Haxonite:

```
$ cd example && haxonite
14:11:42 [INFO] Processing config!
14:11:42 [INFO] Processing config for example_request: RequestConfig { type_: None, method: Some("GET"), path: Some("/"), responses: Some([ResponseConfig { headers: Some(["Content-Type: application/json"]), status: Some(200), response: Some("responses/haxonite.json"), weight: None, delay: None }]) }!
14:11:42 [INFO] Defining route for: / using single type of handler.
14:11:42 [INFO] Processing config for assets_request: RequestConfig { type_: Some("static"), method: None, path: Some("/public"), responses: Some([ResponseConfig { headers: None, status: None, response: Some("assets"), weight: None, delay: None }]) }!
14:11:42 [INFO] Mounting static for: /public using static type of handler.
14:11:42 [INFO] Haxonite running on port: 4000!
```

And let's make a request to our mocked API and see what Haxonite returns:

```
$ curl http://localhost:4000
{
  "haxonite": {
    "version": "0.1.0",
    "authors": "Jakub Matraszek <jakub.matraszek@gmail.com>",
    "logo": "assets/haxonite.png"
  }
}
```

## Disclaimer

This project is in early state of development. A lot of things may change.
There are a couple of TODOs in the source code that are still to be fixed.

**NOTE**: Haxonite started as a project for my private use. I focused on
simplicity, as I usually do not have to implement any sophisticated logic
when mocking API. Hence it lacks the support, for example, for setting
arbitrary HTTP headers in the response or for matching requests based on
parsed parameters or request body. Mocking XMLRPC or SOAP APIs using
Haxonite is not as easy as JSON's, but it's still possible (for example
using `chain` request type). Also, there are no tests yet...

## Changelog

You can check out the changelog
[here](CHANGELOG.md).

## Contributing

1. Fork the project.
2. Create a topic branch.
3. Implement your change.
4. Add an entry to [the changelog](CHANGELOG.md).
5. Commit.
6. Push to the topic branch.
7. Create a pull request.

If you change anything regarding command-line options or config file
format, please update README.md and USAGE.md files. Without this, the
pull request will be rejected.

Please remember to keep your git commit message [short and
informative](http://stopwritingramblingcommitmessages.com/)!

## Questions and issues

Use Github's issue tracker for bug or feature requests. [Drop me
a line](mailto:jakub.matraszek@gmail.com) if you want to ask about
anything that is not a bug/feature request.

## Self-promotion

If you like Haxonite — star this project and share this page. You can also
follow me on [Twitter](https://twitter.com/kubaxvx).

## License

You can check out the full license
[here](https://github.com/jmatraszek/haxonite/blob/master/LICENSE). This
project is licensed under the terms of the **Apache 2.0** license.
