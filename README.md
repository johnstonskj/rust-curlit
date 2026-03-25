# curlit

A Rust command-line interface (CLI) which automates installs of the kind `curl <url>| base`.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)

This tool has 2 modes, an _immediate_ mode to execute a single install, and _file_ mode which reads from a configuration file and can install all tools.

## Install Commands

### Now (Immediate Mode)

The command `curlit now` will immediately execute an install with the provided parameters, without using the configuration file.

```bash
❯ curlit now --help
Fetch and immediately execute a script

Usage: curlit now [OPTIONS] --url <URL>

Options:
  -u, --url <URL>              URL of the script to fetch and run
  -v, --verbose...             Increase logging verbosity
  -q, --quiet...               Decrease logging verbosity
  -s, --shell <SHELL>          Shell to use (defaults to $SHELL or bash)
  -c, --cache-dir <CACHE_DIR>  Directory to cache the downloaded script
  -h, --help                   Print help
```

### Install

The command `curlit install` will attempt to install _all_ entries in the configuration file. The argument `--name NAME` will install _only_ the entry with the name `NAME` from the configuration file.

```bash
❯ curlit install --help
Install all (or a named) entry from the config

Usage: curlit install [OPTIONS]

Options:
  -f, --file <FILE>  Path to config file
  -v, --verbose...   Increase logging verbosity
  -n, --name <NAME>  Name of entry to install (installs all if omitted)
  -q, --quiet...     Decrease logging verbosity
  -h, --help         Print help
```

The command will return an error if a config file **is not** present, or is present and **not writable**. Additionally, it will return an error if an entry name is provided and **no entry with that name exists** in the configuration file.

## Configuration Commands

These commands act on the configuration file(s), allowing the management of reusable install URLs.

### Initialize

The command `curlit init` will create a config and cache directory for curlit and add an empty config file
in the `${XDG_CONFIG_HOME}/curlit` directory.

1. Create the directory `${XDG_CONFIG_HOME}/curlit`.
2. Create an empty file `config.toml` within the previous directory.
3. Create the directory `${XDG_CACHE_HOME}/curlit`.

```bash
❯ curlit init --help
Initialize curlit config and cache directories

Usage: curlit init [OPTIONS]

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

### Add Entry

The command `curlit add` adds a record to the Toml configuration, it does not execute the install of the added entry. It requires the arguments `url`, and `name`, with `file`, `shell`, `cache-dir`, and `type` optional.

```bash
❯ curlit add --help
Add an entry to the config

Usage: curlit add [OPTIONS] --url <URL> --name <NAME>

Options:
  -f, --file <FILE>                  Path to config file
  -v, --verbose...                   Increase logging verbosity
  -q, --quiet...                     Decrease logging verbosity
  -u, --url <URL>                    URL of the script
  -n, --name <NAME>                  Name for this entry
  -N, --command-name <COMMAND_NAME>  Command name for this entry
  -s, --shell <SHELL>                Shell to use when running this script
  -c, --cache-dir <CACHE_DIR>        Directory to cache the downloaded script
  -t, --type <ENTRY_TYPE>            Type of the entry [possible values: cli]
      --force                        Overwrite if entry already exists
  -h, --help                         Print help
```

The command will return an error if a config file **is not** present, or is present and **not writable**.

If the named entry exists within the config file, the command-line argument `--force` will overwrite the existing entry with the new details. If this argument is present and the entry _does not_ exist the argument is ignored. Otherwise an error is returned.

```bash
❯ curlit add --name brew --url https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh
Error: entry named `brew` already exists.
 Help: to overwrite the currrent entry, use the `--force` argument.
```

### Delete An Entry

The command `curlit delete` will delete a named entry from the configuration file.

```bash
❯ curlit delete --help
```

### View Configuration

The command `curlit view` will print out all entries in the configuration file, or a single named entry.

```bash
❯ curlit view --help
View config entries

Usage: curlit view [OPTIONS]

Options:
  -f, --file <FILE>  Path to config file
  -v, --verbose...   Increase logging verbosity
  -n, --name <NAME>  Name of entry to view (shows all if omitted)
  -q, --quiet...     Decrease logging verbosity
      --as-table     Display results as a Markdown table
  -h, --help         Print help
```

```bash
❯ curlit view --name brew
Loaded from: "/Users/s0j0g7m/Projects/curlit/curlit.toml"

[brew]
  url                  = https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh
  shell                = bash
  type                 = cli
```

```bash
❯ curlit view --name brew --as-table
Loaded from: "/Users/s0j0g7m/Projects/curlit/curlit.toml"

| Name         | URL                                                                | Shell | Type | Cached File |
| ------------ | ------------------------------------------------------------------ | ----- | ---- | ----------- |
| brew         | https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh | bash  | cli  | -           |
```

## Cache Commands

These commands operate on the resources stored in the cache.

### View Cache

```bash
❯ curlit cache-view --help
View the current cache directory contents

Usage: curlit cache-view [OPTIONS]

Options:
  -c, --cache-dir <CACHE_DIR>
          Override the standard directory to cache downloaded scripts.
          
          The default value is "${XDG_CACHE_DIR}/curlit".

  -v, --verbose...
          Increase logging verbosity

      --as-table
          Display results as a Markdown table

  -q, --quiet...
          Decrease logging verbosity

  -h, --help
          Print help (see a summary with '-h')
```

```bash
❯ curlit cache-view
Cache directory = "/Users/s0j0g7m/Library/Caches/curlit"

[brew]
  cache-path = "/Users/me/.cache/curlit/brew/install-script"
  src-url    = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"
  fetched.   = Tue, 24 Mar 2026 07:51:16 -0700
  size       = 9.33 kB
  entity-tag = "c5af88b81a64bf3f2e987044a6edc154bd00f32c1b6ac67ef0f3828726aa8d00"
```

### Clear Cache

```bash
❯ curlit cache-clear --help
Clear the current cache directory contents

Usage: curlit cache-clear [OPTIONS]

Options:
  -c, --cache-dir <CACHE_DIR>
          Override the standard directory to cache downloaded scripts.
          
          The default value is "${XDG_CACHE_DIR}/curlit".

  -v, --verbose...
          Increase logging verbosity

  -n, --name <NAME>
          Name of entry to view; shows all if omitted

  -q, --quiet...
          Decrease logging verbosity

  -h, --help
          Print help (see a summary with '-h')
```

### Refresh Cache

TBD

## Utility Commands

### Generate Completions

The command `curlit completions` will write to stdout the shell completions for curlit, you can redirect these
to a file and add to your shell startup.

```bash
❯ curlit completions --help
Generate shell completions

Usage: curlit completions [OPTIONS] [SHELL]

Arguments:
  [SHELL]  Shell to generate completions for (defaults to $SHELL) [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

## Configuration File

The config entry structure describes each record in the Toml file, is used to read and write files in file mode,
and has the following keys:

* **`url`**; The URL to retrieve.
* **`cache-dir`**; (optional) The cache directory to store downloaded files in; default: `${XDG_CACHE_HOME}/curlit`.
* **`shell`**; (optional) The shell to execute with the downloaded script; default: `bash`.
* **`type`**; (optional) The type of installation, currently only `cli` is supported.
* **`command-name`**; (optional) The name of the tool to install/check; defaults to entry name.

The file will be looked for in the following locations, in order.

1. `${XDG_CONFIG_HOME}/curlit/config.toml`
2. `${XDG_DATA_HOME}/curlit/config.toml`
3. `${HOME}/curlit.toml`
4. `${PWD}/curit.toml`

The structure of the configuration file is in the Rust module `config.rs` and comprises a `ConfigFile` struct which
is a newtype wrapper around a vector of `ConfigEntry` structures.

## License(s)

The contents of this repository are made available under the following
licenses:

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-MIT).

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
