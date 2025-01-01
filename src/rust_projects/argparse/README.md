# argparse

argparse is a library for parsing command line arguments. I wrote it for
educational purposes. It is not intended for use in production code.

## Background Details

In C, command line arguments are accessed through the argc and argv parameters
passed into the main function. In Rust command line arguments are accesessed
through std::env::args(). I wanted to know how this worked under the hood on
Linux so here's what I learned during my research. 

DISCLAIMER: My details may be incorrect.

When the execve() syscall is called, the arguments are passed in like so:
> int execve(const char *pathname, char *const _Nullable argv[],
>                  char *const _Nullable envp[]);

execve() then loads the file (referenced in the pathname) and sets up the
process's stack. It pushes argc and argv onto the stack. The process then
accesses the command line arguments from its stack.
([details](https://elixir.bootlin.com/linux/v3.18/source/fs/binfmt_elf.c#L294))

NOTE: Linux supports ELF, AOUT, and scripts as executable files. They are
identified via "magic numbers", which are just the first bytes of the file.
The file's mode must also be marked as executable. 

In C ELF binaries, argc and argv are provided to the main function as parameters.

In Rust ELF binaries, a function defined in the ELF header's .init_array section
sets up the argc and argv variables so that they can be accessed in the Rust
program.
([details](https://github.com/rust-lang/rust/blob/1.52.0/library/std/src/sys/unix/args.rs#L87))
