# General thoughts on metadata shortening project

Useful parts of metadata for extrinsic (transaction) decoding really are types registry and extensions; with these two all transactions could be parsed. Other parts are not really needed and thus could be stripped off. The only other useful for cold devices part of metadata is encoded metadata version (with its type); see below.

Shortened metadata structure: `header registry extensions`

## Metadata paging

We should try to support paging of metadata to load it sequentually into the cold device. If we do this, generally each page should have following structure:

`header [registry part x] [extensions part y]`

where x and y are addresses of corresponding data on a page.

For paging, we should isolate separate chunks of metadata into individually hashable blocks: every page contains only full blocks that could be added to signature as a whole.

If we want to support scrolling back as extrinsic is displayed in partial form to user, we should either be able to check that older page is identical to the one used for signing and shown before (by checking accumulator entity for presence of corresponding hash), or make accumulation process reversible, or store intermediate results, or start anew every time.

Non-shortened metadata would have to perform exactly same operation, thus loading all signers with the same work cold devices do. This should not be something unreasonably complicated.

# `header`

First of all, it is important to note that we **should not try to solve metadata issues that could and should be solved in metadata**, because some day they probably would be solved, in which case our temporary solutions would break and cause lots of confusion.

We could add to header data for faster access to some registry fields (like name and version that are stored within otherwise rarely needed `system` pallet).

The header could also store some data on depth of stripping sequential shortening procedures, although it is not clear whether this would really be useful.

We could include `decimals` and `units` for base currency, although these could be included in metadata itself. Maybe we are trying to solve metadata issues at wrong place.

Shortened metadata potentially could be headerless. Unshortened metadata package probably would be headerless even if its shortened versions are not.

# `registry`

We should remove unused enum variants. If some enum is used several times, we could store all its variants that are actually used. One notorious example of this is set of `batch` calls that list all other calls as their variant.

Shortcutting to types through aliases poses potential problems at questional profit, maybe not worth it.

There are certain notoriously weird types such as era and voting variants.

Era occupies about 1kB of data by explicitly listing 256 variants but its convoluted decoding rules are not even currently reflected in metadata (it does not seem like they ever would be). Thus, era decoding probably should be partially hard-coded in cold devices - as is the case now - and this makes sending whole 1kB for it even more awkward.

Voting options are confusing and misleading; this is quite uncomfortable for governing people - again, the rules for voting could be hardcoded, but on the other side, Gov2 voting brought clearer split variant options. This gives hope that voting options would eventually (soon) become clear indeed and potential temporary hardcoded solution would only cause problems at that point.

# `extensions`

Extensions have some number of empty elements; those could be stripped. Either way, the change would not be too dramatic, but it might be rational.

Era - see above.

# Other

Maybe we should take XCM into account here?

Polkadot Vault should probably keep downloading whole metadata; it's relatively easy - easier than multiframe QRs for each transaction, and less confusing. It would still benefit greatly from this format due to metadata being checked on-chain; whole verifier system will go away.
