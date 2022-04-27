# Rcat - A Rust implementation of the Unix cat command

Part of my Rust learning attempts, this command uses buffers instead of direct memory where possible, hopefully resulting in better performance on large files.

Use in the same way as the native Unix `cat` command to print the content of a file onto the standard output stream.

`rcat ./file/path/file.txt`
