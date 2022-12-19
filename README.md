# Hash delivery network data server

This server is a node that has all the data. Cache servers make requests to this server when
they don't have a key or they want to add new key value pair.

This project uses [tokio](https://docs.rs/tokio/latest/tokio/) as a runtime and
[sled](https://docs.rs/sled/latest/sled/) as it's database to be blazingly fast.

# Configuration

By default server will create configuration file in usual directory for your OS
(on *nix it will be `$XDG_CONFIG_HOME/hdn-data-server/default-config.toml`).
If you wish to use another config you can provide its path through `--config` parameter.
Note that if none such file exists it will be created with default parameters.

Database will also auto create it's files if none exist.

## Default config
```toml
listener_addr = '127.0.0.1:9002'
db_dir = 'data' # Any path can be provided here
```

# Network Deploy

Shortcut: defaults allow you to run one data and one cache server locally without any 
configuration.

1. Run [data server](https://github.com/MetaGigachad/hdn-data-server/) with appropriate configuration
2. Configure [cache servers](https://github.com/MetaGigachad/hdn-cache-server/) with address of data server
3. Run [cache servers](https://github.com/MetaGigachad/hdn-cache-server/) with their configuration

# Communication with cache servers

Supported requests are described by schemes in [messages](crate::message::data_server) and work
similar to how user communicated with cache server, but using postcard encoding and more
optimal layouts.
