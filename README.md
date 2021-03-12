# Nervos Developer Training Course Scripts

This repo contains the example scripts (smart contracts) for the Nervos Developer Training Course.

The content you find here is designed to be used with lessons.

You can find the full developer training course on [GitBook](https://nervos.gitbook.io/developer-training-course/).

**These scripts are for example purposes and should not be used in production!**

## Available Scripts

* **always** - A lock script that always succeeds (unlocks). This is also known as the "Always Success" lock script.
* **ckb500** - A lock script that succeeds when the total input capacity is exactly 500 CKBytes.
* **counter** - A type script that holds a u64 value that must be incremented by 1 on every transfer.
* **data10** - A type script that limits the amount of data in a cell to 10 bytes.
* **datacap** - A type script that limits the amount of data to the amount specified in the args.
* **hashlock** - A lock script that is secured with a Blake2b hash and unlocked with the preimage.
* **ic3type** - A type script that succeeds when the total number of input cells is exactly 3.
* **icclock** - A lock script that does an input capacity check (icc) to verify that at least one input cell has a capacity that matches amount x. The x value is specified in the lock script args.
* **icctype** - A type script that succeeds when the total number of input cells is equal to the number specified in the type script args.
* **jsoncell** - A type script that only allows valid JSON strings to be stored as cell data.
* **never** - A lock script that never succeeds (unlocks). This is also known as the "Always Fail" lock script.
* **occlock** - A lock script that does an output capacity check (occ) to verify that at least x output cells have a capacity that matches amount y. The x and y values are specified as lock script args.

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
