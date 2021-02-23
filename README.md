# Nervos Developer Training Course Scripts

This repo contains the example scripts (smart contracts) for the Nervos Developer Training Course.

The content you find here is designed to be used with lessons.

You can find the full developer training course on [GitBook](https://nervos.gitbook.io/developer-training-course/).

## Usage

Build all contracts (debug):

``` sh
capsule build
```

Run all tests:

``` sh
capsule test
```

Build all contracts (release):
``` sh
capsule build --release
```

Build a specific contract (debug):
``` sh
capsule build --name counter
```

Build a specific contract (release):
``` sh
capsule build --name counter --release
```
