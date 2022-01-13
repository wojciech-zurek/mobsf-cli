# mobsf-cli

mobsf-cli is a wrapper for the Mobile Security Framework (MobSF) REST-API

## Manual installation

```fish
git clone git@github.com:wojciech-zurek/mobsf-cli.git
cd mobsf-cli
cargo build --release
sudo cp target/release/mobsf-cli /bin/mobsf-cli
mobsf-cli --help
```

## Usage

```fish
mobsf-cli 0.1.0
Wojciech Zurek <mail@wojciechzurek.eu>
mobsf-cli app

USAGE:
    mobsf-cli [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -a <api_key>        Api key/token (overrides MOBSF_API_KEY env)
    -h, --help          Print help information
    -s <server>         Server, example: http://localhost:8000 (overrides MOBSF_SERVER env)
    -V, --version       Print version information

SUBCOMMANDS:
    ci        For CI/CD stages. Upload a file, auto start scan, save reports, check scores.
    delete    Delete scan.
    help      Print this message or the help of the given subcommand(s)
    play      Upload a file and auto start scan.
    report    Get report.
    scan      Scan a file.
    scans     Display recent scans.
    source    View source files.
    upload    Upload a file.

```

## Server and api key

You can set server and api key:

- as command options (higher order),
    - `-a <api_key>`
    - `-s <server>`
- as environment variables (lower order)

#### Environment variables

You can set env for api and server config:

- `MOBSF_API_KEY` - for api key,
- `MOBSF_SERVER` - for server

```fish
MOBSF_API_KEY="ed...c4" mobsf-cli scans'
```