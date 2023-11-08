# Milestone 2

## 2.1. Basic algorithm to generate a shortened metadata and associated Merkle tree as proof-of-concept code example

The basic algorithm is implemented in [`metadata-shortener`](https://github.com/Alzymologist/metadata-shortener) crate.

## 2.2. Refined draft for mock shortened metadata

Protocol draft could be found at metadata-shortener crate [examples](https://github.com/Alzymologist/metadata-shortener) (see also documentation).

## 2.3. Feasibility analysis and possible caveats/solutions for offline devices implementations

Shortening algorithm draft was fully included in Kampela device and successfully demonstrated at Sub0 and shipped with Kampela protoboard firmware package.

The verifying code depends on blake3 functionality. We have confirmed that blake3 can be used in both Ledger and Kampela devices; single-thread implementation allows Ledger device to handle metadata chunks one by one without constructing large tree structure. Kampela has sufficient memory to store whole proof mountain range.

Merkle tree proof was chosen over other accumulators after some prototyping due to:
1. Higher maturity and better maintenance of existing implementations;
2. Optimization for smaller transaction size (which is more popular use case) and more flexible modular structure of proof;
3. Higher code reuse;
4. Simpler math (simplifying re-implementations);
5. Sufficiency for the task.

## 2.4. Early implementation of a generic app with metadata support

The early implementation was demonstrated on Sub0 event.
