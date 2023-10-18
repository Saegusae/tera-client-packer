# tera-client-packer
A CLI utility to compress and fragment game client files for TERA Online.

## What does it do?
This is a tool that packs game client files for distribution through a launcher or installer. Created for TERA online but can be used for anything really.

## Why does it exist?
I've been messing with how [Menma's TERA](https://discord.gg/mtdream) manages installation and patching through their client while learning the Rust language. Noticed a few caveats with their current implementation that puzzled me:

* Files are not compressed even though they planned for implementation (evident through their manifest having a flag field for compression) resulting in an increased size of 20.57% thats give or take 9~GB more data to download.
* Source files are fragmented in a way that they can be split between multiple package files in an attempt to make parts equal in byte length. This is acceptable in most cases but severely hurts multi-threaded unpacking performance as workloads have to wait the write-lock on the split destination file when you are processing (unpacking) these files from packages.
