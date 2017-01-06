# Usage

# Table of Contents

   * [Command line](#command-line)
   * [Config file](#config-file)
      * [Server configuration](#server-configuration)
      * [Requests and responses configuration](#requests-and-responses-configuration)
         * [Request configuration: method (optional)](#request-configuration-method-optional)
         * [Request configuration: path (mandatory)](#request-configuration-path-mandatory)
         * [Request configuration: responses (mandatory)](#request-configuration-responses-mandatory)
         * [Request configuration: type (optional)](#request-configuration-type-optional)
         * [Response configuration: status (optional)](#response-configuration-status-optional)
         * [Response configuration: content_type (optional)](#response-configuration-content_type-optional)
         * [Response configuration: response (mandatory)](#response-configuration-response-mandatory)
         * [Response configuration: weight (optional)](#response-configuration-weight-optional)


## Command line

Create a new project by running `haxonite new project_name` and then run
`haxonite` in the project directory to start Haxonite.

Run `haxonite help` and `haxonite help new` to find out more about
available options.

## Config file

Haxonite uses `config.toml` file as a configuration file. This file has
two sections: `[server]` (for server-related settings) and `[requests]`
(for defining requests and responses assigned to them).

### Server configuration

Server section may be used to configure two basic server options: hostname
and port on which Haxonite runs. Default server section looks like this:

```
[server]
port = 4000
host = "localhost"
```

Please note that the value specified as a port should be an integer and
value specified as a hostname should be a string. You may need root rights
if you want to run Haxonite on a port number less than 1024. You may omit
those options in the config file and the default values (`4000` and
`"localhost"`) will be used. You may also overwrite them from
command-line. Please refer to `haxonite help` for more details on that.

### Requests and responses configuration

`[requests]` section allows you to define as many requests as you want.
You define a request like this `[requests.name_of_the_request]` and then
specify its options. `name_of_the_request` is just a name that is used
internally, so you are free to pick anything that suits you. Each request
has the following options: `type`, `method`, `path` and `responses`.
Example:

```
[requests.some_random_resources]
type = "random"
method = "GET"
path = "/api/v1/random_resources"
[[requests.some_random_resources.responses]]
status = 200
content_type = "application/json"
response = "responses/resources.json"
weight = 95
[[requests.some_random_resources.responses]]
status = 500
content_type = "application/json"
response = "responses/error.500.json"
weight = 5

[requests.assets_request]
type = "static"
path = "/public"
[[requests.assets_request.responses]]
response = "assets"
```

#### Request configuration: `method` (optional)

This specifies the HTTP method. All standard HTTP methods are allowed.
Defaule value: `"GET"`. Example:

```
method = "GET"
```

#### Request configuration: `path` (mandatory)

This specifies the path. You may use the `:param` notation for params that
may vary. Example:

```
path = "/api/v1/resources/:id"
```

#### Request configuration: `type` (optional)

This specifies the type of the request. Request may have one of the
following types:
+ `"single"`: every request will return the same response;
+ `"random"`: every request may return radnom response defined in
  `"responses"` array;
+ `"roundrobin"`: consecutive requests will return consecutive responses
  defined in `responses` array. After returning the last one, Haxonite will
  start from the beginning;
+ `"chain"`: similar to `roundrobin`, the only difference is that
  after reaching the last response defined in `responses` array, Haxonite
  will keep on returning the last response;
+ `"static"`: used to serve static files.

Default value: `"single"`. Example:

```
type = "single"
```

#### Request configuration: `responses` (mandatory)

This option is an array of all possible responses for a given request. `type`
parameter specifies how they are returned. Example:

```
[[requests.show.responses]]
status = 200
content_type = "application/json"
response = "responses/resource.json"
[[requests.show.responses]]
status = 500
content_type = "application/json"
response = "responses/error.500.json"
```

#### Response configuration: `status` (optional)

This specifies the HTTP status that the response will have. Default value:
`200`. Example:

```
status = 200
```

#### Response configuration: `content_type` (optional)

This specifies the "Content-Type" HTTP header that the response will have.
Default value: `"application/json"`. Example:

```
content_type = "application/json"
```

**NOTE**: This will probably change in the future, when Haxonite gains
a possibility to configure any HTTP header.

#### Response configuration: `response` (mandatory)

This specifies the path to file with the actual response body or (for
`static` request) a path to static files that will be served. Example:

```
response = "responses/resource.json"
```

#### Response configuration: `weight` (optional)

This specifies the weight that each response has. Setting this only makes
sense for a `random` request. Default value: `1`. Example:

```
weight = 95
```
