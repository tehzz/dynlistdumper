# DynListDumper
A tool to dump the "dynlists" used on Super Mario 64's Press Start screen (the one with the head.)

## Usage
```
dynlistdump 1.0.0
A tool to help dump a binary SM64 head screen dynlist into a set of asm macros

USAGE:
    dynlistdump <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    asm     Create the set of gas macros needed for assembling a dumped dynlist
    c       Create the set of cpp macros needed for initializing a dynlist cmd struct
    dump    Dump a binary dynlist into a list of gas macros
    help    Prints this message or the help of the given subcommand(s)
```
### GAS style macros
```
Create the set of gas macros needed for assembling a dumped dynlist

USAGE:
    dynlistdump asm [output]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <output>    output file, or stdout if not present
```
### C style header
```
Create the set of cpp macros needed for initializing a dynlist cmd struct

USAGE:
    dynlistdump c [output]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <output>    output file, or stdout if not present
```
### Actual Binary Dumping
```
USAGE:
    dynlistdump dump [FLAGS] <input> [ARGS]

FLAGS:
    -c, --c-macros      print out the C macros instead of gas
    -h, --help          Prints help information
    -i, --info          print info about a list, rather than dumping the bytes
    -r, --raw-values    print out the raw values of cmd as a comment
    -V, --version       Prints version information

ARGS:
    <input>     input binary file to read dynlist from
    <offset>    offset to start of dynlist
    <output>    output file, or stdout if not present
```
