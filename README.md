# Monitrust

## Motivation

Monitrust aims to provide a minimal and easy-to-use service for monitoring
servers and creating alerts. The monitoring is done on the machine itself (not
by connecting to a another server). This makes it convenient to use for e.g. a
self-hosted server.

Compared to e.g. Grafana, the setup of Monitrust is simpler, and consumes a lot
less resources than their Grafana Agent.

### Goals

1. Small memory footprint
2. Straightforward configuration

### Non-goals

This is no Grafana replacement. Complex configuration and reporting mechanisms
are not the point here.

## How to use

### Configuration

Monitrust uses two configuration files: `reporters.json` and `watchers.json`.

The `reporters.json` file contains the configuration needed to report alerts.
**This file will likely contain API keys of some sort, and should be stored
securely!**

The `watchers.json` contains the things that should be monitored, as well as the
thresholds at which alerts should trigger.

You can use the `reporters.json.example` and the `watchers.json.example` files
to see what options exist (i.e. what alerts and reporters exist).

### Deployment

Monitrust supports generating `deb` packages with `systemd` unit files, using
[cargo deb](https://github.com/kornelski/cargo-deb):
```bash
cargo deb
```

Then you can simply upload your package and install it.

## Contributing

The repository is currently tailored to my own needs, but extending the
capabilities is by itself no problem. Some ideas of possible improvements:

* add `sshd` log-in watcher to warn whenever a user logs in
* add more reporters (some might like Discord I guess), web-hook, IRC, whatever
  (hopefully one day Signal offers some bot API)
* add more deployment options (besides Debian + systemd)
