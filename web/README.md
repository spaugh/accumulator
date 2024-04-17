# API Documentation

## Overview

This API provides a simple interface for managing leaves and retrieving Merkle Mountain Range (MMR) proofs and peaks based on block IDs. The term "block" is used a bit loosely, as there is a new block for every leaf added (there is no more quantization) - it's just a convenient term for describing the state of the accumulator.

## Routes

### `POST /leaves`
Adds a new leaf to the MMR. Each new leaf results in the creation of a new block.

### `GET /blocks/:id/peaks`
Retrieves the peaks for the specified block ID.

### `GET /blocks/:id/proofs/:index`
Retrieves the proof for a leaf at the specified index within the given block ID.