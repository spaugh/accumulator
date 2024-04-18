# API Documentation

## Overview

This tool is intended to be a demonstration of the public use of an incremental cryptographic accumulator. An example usage might be for proving membership in a set, making some claim that you later reveal, or even as a blockchain.

There are two components to this project - a REST API and a CLI. One can imagine that the users of those two components would be separate parties.

More specifically, the API provides a simple interface for managing leaves and retrieving Merkle Mountain Range (MMR) proofs and peaks based on block IDs. The term "block" is used a bit loosely, as there is a new block for every leaf added (there is no more quantization) - it's just a convenient term for describing the state of the accumulator.

## Getting started

There is a simple bash script that simulates the interaction between the parties. To run it:

```
./run.sh
```

## Routes

### `POST /leaves`
Adds a new leaf to the MMR. Each new leaf results in the creation of a new block.

### `GET /blocks/:id/peaks`
Retrieves the peaks for the specified block ID.

### `GET /blocks/:id/proofs/:index`
Retrieves the proof for a leaf at the specified index within the given block ID.

