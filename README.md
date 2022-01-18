# mobsf-cli

mobsf-cli is a wrapper for the Mobile Security Framework (MobSF) REST-API. Created especially for Continuous
Integration (CI) / Continuous Delivery (CD) stages. You can use only use one command to upload a file, auto start scan,
save reports, check scores.

[Mobile Security Framework (MobSF)](https://github.com/MobSF/Mobile-Security-Framework-MobSF) is an automated,
all-in-one mobile application (Android/iOS/Windows) pen-testing, malware analysis and security assessment framework
capable of performing static and dynamic analysis.

## Releases

Go to [releases page](https://github.com/wojciech-zurek/mobsf-cli/releases) and fetch latest release.

### Install (linux x86_64)

```fish
wget https://github.com/wojciech-zurek/mobsf-cli/releases/download/v0.1.0/mobsf-cli-x86_64-unknown-linux-gnu.tar.gz 
tar -xvf mobsf-cli-x86_64-unknown-linux-gnu.tar.gz
sudo mv mobsf-cli /usr/local/bin/mobsf-cli
sudo chmod +x /usr/local/bin/mobsf-cli
```

## Manual installation

```fish
git clone git@github.com:wojciech-zurek/mobsf-cli.git
cd mobsf-cli
cargo build --release
sudo cp target/release/mobsf-cli /usr/local/bin/mobsf-cli
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

### Example usage

```fish
# Upload a file to MobSF server
mobsf-cli upload path/to/example.apk

# Scan a file
mobsf-cli scan apk example.apk <hash>

# Upload a file and auto start scan
mobsf-cli play path/to/example.apk

# Fetch scan result (report)
mobsf-cli report pdf <hash>
mobsf-cli report json <hash>

# Display recent scans
mobsf-cli scans

# Delete scan result
mobsf-cli delete <hash>
```

## CI/CD usage

`mobsf-cli ci` combines:

- upload a file,
- start scan,
- generate reports in pdf and json format,
- check scan scores (cvss, security score, trackers) and rise an error if scores are wrong

```
# help
mobsf-cli ci --help

mobsf-cli-ci 
For CI/CD stages. Upload a file, auto start scan, save reports, check scores.

USAGE:
    mobsf-cli ci [OPTIONS] -p <path_to_save> <file_path>

ARGS:
    <file_path>    

OPTIONS:
    -a <api_key>             Api key/token (overrides MOBSF_API_KEY env)
    -c <cvss>                Above this score rise a cvss error. 0.0-10.0 [default: 3.9]
    -h, --help               Print help information
    -p <path_to_save>        Path to directory to save reports (pdf and json).
    -r                       Rescan a file
    -s <server>              Server, example: http://localhost:8000 (overrides MOBSF_SERVER env)
    -t <trackers>            Above this score rise a trackers error. 0-407 [default: 0]
    -u <security>            Below this score rise a security error. 0-100 [default: 71]

```

```fish
mobsf-cli ci path/to/example.apk -p path/to/save/reports -c 5.5 -u 48 -t 2
...
Validating scan scores...
Error: CVSS score [6.6] is to high. Max: 5.5!
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
MOBSF_API_KEY="ed...c4" MOBSF_SERVER="https://su...com:8000" mobsf-cli scans
```