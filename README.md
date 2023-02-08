# [Nostr] Vanity Address Generator

> CLI tool to generate vanity addresses for Nostr

[![cicd][cicd-badge]][cicd] [![release][release-badge]][releases] [![license][license-badge]](LICENSE)

## Usage

Download the latest release from the [releases] page or build from source (see below), then run the binary.

```
$ ./nostrgen --help
Usage: nostrgen [OPTIONS] --prefix <PREFIX>

Options:
  -p, --prefix <PREFIX>  The address prefix to match
  -s, --suffix <SUFFIX>  The address suffix to match (optional)
  -c, --cores <CORES>    Cpu cores to use (default: cpu_cores/2)
  -h, --help             Print help
```

### Example

```
$ ./nostrgen -p 7777
[#] Start searching with prefix npub17777 (difficulty est.: 1048576)
[!] Result:
secret_key:  3d81e7db2e250685b8246def93cb9d29b7a4c73139b4c0ec37e6df63f9a86e7b (hex)
secret_key:  nsec18kq70kewy5rgtwpydhhe8jua9xm6f3e38x6vpmphum0k87dgdeashjjxyw
public_key:  npub17777wunn2aq5megnxlnckcfhz2w4zejrvl4nnnha2lfl9lh9qzaqpr4jt4

$ ./nostrgen -p 000 -s 00 -c 10
[#] Start searching with prefix npub1000 and suffix 00 (difficulty est.: 33554432)
[+] Total 4331377 keys in 0.5 mins (144379.2 keys/s)
[!] Result:
secret_key:  4cb87474993b0693cd24eabb40a2affc54fd3fc31bb7cb18224456b49e81b2c4 (hex)
secret_key:  nsec1fju8gaye8vrf8nfya2a5pg40l320607rrwmukxpzg3ttf85pktzqpxp28t
public_key:  npub100007s69jra7w0gfulcreacudxdqwc60dqaskfre0fdkfa0cwe5su5pg00
```

### Build from source

You need to have Rust toolchain installed in your system, see [rustup.rs] for
the install guide if you don't have it.

```
# Get the source code and build
$ git clone https://github.com/chawyehsu/nostr-vanity-address-generator
$ cd nostr-vanity-address-generator
$ cargo build --release

# Run the binary
$ ./target/release/nostrgen --help
```

## Knowledge

### Speed

Though multithreading has been supported, it depends on your hardware, CPU
specifically and the prefix/suffix you are looking for. Longer prefixes/suffixes
will take longer to find, the difficulty increases at an exponential rate. There
should be room for performance improvements I believe, so PRs are welcome.

### Character Set

Nostr public key is encoded in `bech32` format, which uses a character set excepting
`b`, `i`, `o` and `1`, hence you won't be able to get an address with these characters.

For more information about Nostr's address format, you may read about Nostr's [NIP-19]
document.

### Security

This program will neither store nor will steal your secret key, but please
**DON'T TRUST, VERIFY**. Check out the code and build from source yourself, run
it offline if you are concerned.

## License

**nostr-vanity-address-generator** © [Chawye Hsu](https://github.com/chawyehsu). Released under the [Apache-2.0](LICENSE) License.  

> [Blog](https://chawyehsu.com) · GitHub [@chawyehsu](https://github.com/chawyehsu) · Twitter [@chawyehsu](https://twitter.com/chawyehsu)

[Nostr]: https://github.com/nostr-protocol/nostr
[cicd-badge]: https://img.shields.io/github/actions/workflow/status/chawyehsu/nostr-vanity-address-generator/cicd.yml?style=flat-square
[cicd]: https://github.com/chawyehsu/nostr-vanity-address-generator/actions/workflows/cicd.yml
[release-badge]: https://img.shields.io/github/v/release/chawyehsu/nostr-vanity-address-generator?style=flat-square
[releases]: https://github.com/chawyehsu/nostr-vanity-address-generator/releases/latest
[license-badge]: https://img.shields.io/github/license/chawyehsu/nostr-vanity-address-generator?style=flat-square
[rustup.rs]: https://rustup.rs
[NIP-19]: https://github.com/nostr-protocol/nips/blob/master/19.md
