# Milestone 1

THIS IS A DRAFT

## 1.1. Publish refined requirements for protocol

TODO

All metadata blocks should be atomic hashable objects with their hashes presented in arbitrary order.

Hash function should be already in Substrate/cold device codebase

Hash function should be cryptographically strong.

Ordering of metadata blocks should be performed based on their content.

Various levels of metadata stripping should be available per decision of cold device design.

Unused variants should be stripped from enum-like objects.

Note that it follows that each metadata unit would be hashed with `blake2` or `blake3`

## 1.2. Publish first draft of mock shortened metadata for parallel development

As it turns out, it was way easier to write an actual shortening code than attempt to write mock shortened metadata manually. Example of shortening prototype can be found in [substrate-parser](https://github.com/Alzymologist/substrate-parser/blob/main/examples/automated_short_meta.rs) repository. By running `cargo run --example automated_short_meta` in that repository, basic statistics on metadata and content of transaction as decoded using shortened version could be seen.

```
length of basic meta: 300431
length of shortened meta: 1479
TransactionParsed { call_result: Ok(Call(PalletSpecificData { pallet_info: Info { docs: "", path: Path { segments: ["pallet_utility", "pallet", "Call"] } }, variant_docs: "", pallet_name: "Utility", variant_name: "batch_all", fields: [FieldData { field_name: Some("calls"), type_name: Some("Vec<<T as Config>::RuntimeCall>"), field_docs: "", data: ExtendedData { data: SequenceRaw(SequenceRawData { element_info: [Info { docs: "", path: Path { segments: ["westend_runtime", "RuntimeCall"] } }], data: [Call(Call(PalletSpecificData { pallet_info: Info { docs: "", path: Path { segments: ["pallet_staking", "pallet", "pallet", "Call"] } }, variant_docs: "", pallet_name: "Staking", variant_name: "bond", fields: [FieldData { field_name: Some("value"), type_name: Some("BalanceOf<T>"), field_docs: "", data: ExtendedData { data: PrimitiveU128 { value: 1061900000000, specialty: Balance }, info: [] } }, FieldData { field_name: Some("payee"), type_name: Some("RewardDestination<T::AccountId>"), field_docs: "", data: ExtendedData { data: Variant(VariantData { variant_name: "Staked", variant_docs: "", fields: [] }), info: [Info { docs: "", path: Path { segments: ["pallet_staking", "RewardDestination"] } }] } }] })), Call(Call(PalletSpecificData { pallet_info: Info { docs: "", path: Path { segments: ["pallet_staking", "pallet", "pallet", "Call"] } }, variant_docs: "", pallet_name: "Staking", variant_name: "nominate", fields: [FieldData { field_name: Some("targets"), type_name: Some("Vec<AccountIdLookupOf<T>>"), field_docs: "", data: ExtendedData { data: SequenceRaw(SequenceRawData { element_info: [Info { docs: "", path: Path { segments: ["sp_runtime", "multiaddress", "MultiAddress"] } }], data: [Variant(VariantData { variant_name: "Id", variant_docs: "", fields: [FieldData { field_name: None, type_name: Some("AccountId"), field_docs: "", data: ExtendedData { data: Id(08264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d (5CFPcUJg...)), info: [Info { docs: "", path: Path { segments: ["sp_core", "crypto", "AccountId32"] } }] } }] }), Variant(VariantData { variant_name: "Id", variant_docs: "", fields: [FieldData { field_name: None, type_name: Some("AccountId"), field_docs: "", data: ExtendedData { data: Id(aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934 (5G1ojzh4...)), info: [Info { docs: "", path: Path { segments: ["sp_core", "crypto", "AccountId32"] } }] } }] }), Variant(VariantData { variant_name: "Id", variant_docs: "", fields: [FieldData { field_name: None, type_name: Some("AccountId"), field_docs: "", data: ExtendedData { data: Id(9ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d (5FZoQhgU...)), info: [Info { docs: "", path: Path { segments: ["sp_core", "crypto", "AccountId32"] } }] } }] })] }), info: [] } }] }))] }), info: [] } }] })), extensions: [ExtendedData { data: Composite([]), info: [Info { docs: "", path: Path { segments: ["frame_system", "extensions", "check_non_zero_sender", "CheckNonZeroSender"] } }] }, ExtendedData { data: Composite([]), info: [Info { docs: "", path: Path { segments: ["frame_system", "extensions", "check_spec_version", "CheckSpecVersion"] } }] }, ExtendedData { data: Composite([]), info: [Info { docs: "", path: Path { segments: ["frame_system", "extensions", "check_tx_version", "CheckTxVersion"] } }] }, ExtendedData { data: Composite([]), info: [Info { docs: "", path: Path { segments: ["frame_system", "extensions", "check_genesis", "CheckGenesis"] } }] }, ExtendedData { data: Composite([FieldData { field_name: None, type_name: Some("Era"), field_docs: "", data: ExtendedData { data: Era(Era::Mortal(64, 5)), info: [Info { docs: "", path: Path { segments: ["sp_runtime", "generic", "era", "Era"] } }] } }]), info: [Info { docs: "", path: Path { segments: ["frame_system", "extensions", "check_mortality", "CheckMortality"] } }] }, ExtendedData { data: Composite([FieldData { field_name: None, type_name: Some("T::Index"), field_docs: "", data: ExtendedData { data: PrimitiveU32 { value: 2, specialty: Nonce }, info: [] } }]), info: [Info { docs: "", path: Path { segments: ["frame_system", "extensions", "check_nonce", "CheckNonce"] } }] }, ExtendedData { data: Composite([]), info: [Info { docs: "", path: Path { segments: ["frame_system", "extensions", "check_weight", "CheckWeight"] } }] }, ExtendedData { data: Composite([FieldData { field_name: None, type_name: Some("BalanceOf<T>"), field_docs: "", data: ExtendedData { data: PrimitiveU128 { value: 0, specialty: Tip }, info: [] } }]), info: [Info { docs: "", path: Path { segments: ["pallet_transaction_payment", "ChargeTransactionPayment"] } }] }, ExtendedData { data: Tuple([]), info: [] }, ExtendedData { data: PrimitiveU32 { value: 9430, specialty: SpecVersion }, info: [] }, ExtendedData { data: PrimitiveU32 { value: 7, specialty: TxVersion }, info: [] }, ExtendedData { data: GenesisHash(0xe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e), info: [Info { docs: "", path: Path { segments: ["primitive_types", "H256"] } }] }, ExtendedData { data: BlockHash(0x5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff), info: [Info { docs: "", path: Path { segments: ["primitive_types", "H256"] } }] }, ExtendedData { data: Tuple([]), info: [] }, ExtendedData { data: Tuple([]), info: [] }, ExtendedData { data: Tuple([]), info: [] }] }
```

Notice, that metadata for a complex composite batched transaction fits in under 1.5kB, as desired. The shortened metadata is saved in `for_tests/westend9430_short_for_transaction`, original metadata is taken from `for_tests/westend9430`. You can replace original metadata and hardcoded transaction with any others, just remember to change genesis hash that is not part of metadata.

The only part missing there is information on proofing that might be needed in some possible implementation. A placeholder of 256 bits or something similar could be appended to metadata for modeling; we do not know the nature of that part yet, so let's leave it at this.

## 1.3. Estimate shortened metadata size frames based on information theory

The current metadata size is around `S_original = 0.5 MB`. Most of this structure is type registry, with other components size being relatively negligible.

Type registry for Westend currently contains around `N_records_original = 800` records, with `N_records_compound = 300` of those being compound enums and the rest `N_records_primitive = 500`. Per variant stripping requirement and with average number of variants being on the order of `N_variants = 10`, this results in `N_units_total = N_records_primitive + N_records_compound * N_variants = 500 + 300 * 10 = 3500` units. Let's use 5000..10000 as safe reasonable pessimistic estimate.

An average unit contains roughly `S_unit = 100 bytes` of data after docs are stripped (as measured in tests).

Reasonable hash size H256 is `S_hash = 32 bytes`. Metadata for hash addressing should generally fit in `S_hash_meta = 1 byte`

Thus, total metadata separated into units would occupy `S_units_total = 500..1000kB` (agrees with original metadata size). Full set of metadata hashes would occupy `S_hashes_total = 150..350 kB`, and when constructed into Merkle tree it would result in 'D_total = 12..13` layers.

Typical metadata for a transaction would reasonably require `N_units_per_tx = 10..30` units.

Transfer of one unit with Merkle tree proof would require transfer of unit its content with S_unit data alongside with its neighbouring Merkle vertex set. Transfer of the first unit would require full depth of leaves D_total to be transferred along with the data; all subsequent units would be diverging from previously sent ones thus resulting in smaller data chunks. With `N_units_per_tx << N_units_total`, we could estimate that point of divergence would be uniformly distributed in `1..D_total`, thus total amount of hashes sent would be roughly `N_hashes_per_tx_merkle = N_units_per_tx * (D_total/2) = 60..200`, or `S_hashes_per_tx_merkle = N_hashes_per_tx_merkle * (S_hash + S_hash_meta) ~= 2000..7000 bytes`. This, however, does not take into account the fact that embedded devices would be struggling to store fractions of hash table simultaneously while computing these hashes, storing metadata units, and decoding transaction. In some implementation, this would result in double transfer requirement (which is also close to worst-case estimate where units are distributed as sparsely as possible in the tree) `N_hashes_per_tx_merkle_max ~= N_units_per_tx * D_total = 400`

It is important to note here, however, that Merkle tree usage here is not exactly justified - all unused branches serve no other purpose but to prove to the cold device, that hot side indeed can factor the final hash by the hash of unit metadata record. Furthermore, information content of final signature (32 bytes) is much lower than size of transferred hashes, which hints at certain inefficiency of the scheme here. Below we propose simple strategy that could result in same security level at much lower cost while providing some extra benefits, as well as justification for this optimization approach.

## 1.4. Coordination

Teams have held weekly meetings, working communication through Matrix channel, and have signed formal contract agreements to please the non-crypto regulators.

## 1.5. Analyze alternative hashing/storage strategies to Merkle tree

### Serial hashing

One possible approach to Merkle tree hashing could be sequential hashing scheme. Sequential hashing [could be equivalent](https://doi.org/10.1007/s10207-013-0220-y) to thee hashing, with benefits of lower memory/transfer size requirements and downsides of parallelisation and lower overhead of storage modifications, both of the latter are on no interest to us while the memory overhead in indeed undesireable. This would require `N_hashes_per_tx_sequence = N_units_per_tx + 1 = 11..31` units, or `S_hashes_per_tx_sequence = N_units_per_tx * S_hash ~= 350..1000 bytes` if hash function has some kind of associative properties, or total set of hashes otherwise. Here, ordering of elements is important, and associativity is difficult to achieve together with cryptographic strength.

### Symmetric accumulators

With symmetric accumulators, validity of any metadata unit would be trivial to check in O(1) time and then use the accumulator value in signing.

Typical symmetric accumulators (like Bloom filter based ones) have a very clear memory limitation of `S_accumulator_symm = N_total * log(1/e) bits` where `e` is false positive rate. With parameters listed above, this results in few kB even for small numbers like `e=1/1000`, so no benefits are to be found here.

### Asymmetric accumulators

Typical asymmetric accumulators could be used as proof of unit membership; typical usable accumulator size for 32-byte units is 256 byte. Unfortunately, membership proofs should be computed and sent along with data, with typical proof size on order of 800 bytes.

These sizes all fall within the same range, so there is not much to win by replacing tested-and-tried Merkle trees with these schemes. These sizes are overhead for the protocol features that we do not really need, so below we propose a very lean scheme that has minimum of features and at the same time solves the problem, with almost no extra code introduced.

### Proposed solution

We propose to use Benaloh/de Mare scheme with GF(2^256) as hash function, store accumulator value on chain for signature checks and use it to force hot side to prove, that it can indeed factor a publicly known number into metadata unit hash.

