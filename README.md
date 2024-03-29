# tera-client-packer

A CLI utility to compress and fragment game client files for TERA Online. Allows users to download and unpack the game faster, saving bandwidth and time. The tool also exposes a rust library to perform the same actions programatically.

## How does it work?

```
tera-client-packer pack [OPTIONS] <INPUT_DIR>

Arguments:
  <INPUT_DIR> - Parent directory where the client files are located

Options:
  -w, --workers <usize>             Worker count                              [default: 8]
  -n, --package-name <string>       Output package name                       [default: client]
  -e, --package-extension <string>  Output package extension                  [default: cabx]
  -s, --package-size <u64>          Output fragment size in MB                [default: 500]
  -o, --output-dir <Path>           Path where package files will be dumped   [default: ./packed]
  -m, --manifest <Path>             Path where manifest will be stored        [default: ./packed/_manifest.json]

Example:
  tera-client-packer pack -o ./out -m ./out/_manifest.json -w 16 -n client -s 500 -e bin ./GameClient
```

```
tera-client-packer unpack [OPTIONS] <OUTPUT_DIR>

Arguments:
  <OUTPUT_DIR> - Top Level directory where the client files will be unpacked

Options:
  -i, --input-dir <Path>  Input directory where package files are contained [default: ./packed]
  -m, --manifest <Path>   Define a custom manifest file for unpacking       [default: ./_manifest.json]
  -w, --workers <usize>   Thread count for multithreaded use                [default: 8]

Example:
  tera-client-packer unpack -i ./tmp -m ./tmp/_manifest.json -w 16 ./GameClient
```

The program reads around `package-size * 2` amount of data for every thread, so if package size is set to `500mb` the total memory usage for **8 workers** will be around `8-10 GB`.

## What does it do?

This is a tool that packs game client files for distribution through a launcher or installer. Created for TERA online but can be used for anything really.

## Why does it exist?

I've been messing with how [Menma's TERA](https://discord.gg/mtdream) manages installation and patching through their client while learning the Rust language. Noticed a few caveats with their current implementation that puzzled me:

- Files are not compressed even though they planned for implementation (evident through their manifest having a flag field for compression) resulting in an increased size of 20.57% thats give or take 9~GB more data to download. ~~While currently also not implemented in my solution I plan to support Gzip and/or LZMA compression in the near future~~ Gzip compression is implemented and enabled default now.
- Source files are fragmented in a way that they can be split between multiple package files in an attempt to make parts equal in byte length. This is acceptable in most cases but severely hurts multi-threaded unpacking performance as workloads have to wait the write-lock on the split destination file when you are processing (unpacking) these files from packages.

## Multi-thread Implementation Details

I currently have a working prototype for multi-threaded io and compression but it could be better optimized. The program reads files sequentially and stores the buffer for processing every `package-size` bytes reached. For memory optimization purposes, the program manages a thread pool of `worker_count + 1` and a buffer queue length of `worker_count`. So the amount of memory used at all times will stay consistent at around `worker_count * package_size * 2` which of course includes the stored task buffer and concurrently processing byte streams.

Probably could have done it more efficient with memory using some other practice but this is heaps better than running write operations sequentially.

## Benchmarks

All tests were run on client files for patch 100.02, clean Gameforge release with ReleaseRevision.txt md5 hash `0396410868EDE6E05F8DEDC5142E93EB` and `package-size` option set to `500mb`

### Packer

| Runtime                        | Compression     | Duration  | Result            |
| ------------------------------ | --------------- | --------- | ----------------- |
| Single-Threaded                | No Compression  | 1m37s     | 66.9 GB (100.00%) |
| Single-Threaded                | Deflate         | 2h32m44s  | 59.5 GB (88.94%)  |
| Multi-Threaded (16 Threads)    | Gzip (Defaults) | 37m18s    | 58.6 GB (87.29%)  |
| \* Multi-Threaded (16 Threads) | Gzip (Defaults) | **3m33s** | 58.6 GB (87.29%)  |

`* Optimised release target build`

### Unpacker

| Runtime                     | Compression  | Duration |
| --------------------------- | ------------ | -------- |
| Multi-Threaded (16 Threads) | Gzip (Lv. 6) | 5m50s    |

## Known Issues

- Progress bar is very uninteractive and often appears as if the program hanged even though it hasn't.
- There is no enforcement of memory limits, the program generally consumes around `package-size * workers * 2.10` MB of memory, use these options with caution.
- No current package hashing functionality, which is a must if intending to distribute the output.
- Output directory or name of the manifest file is currently not customizable, it defaults to `cwd` and `_manifest.json` respectively
