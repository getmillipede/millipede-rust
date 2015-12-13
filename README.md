# millipede-rust
:bug: Print a beautiful millipede (written in rust)

## Usage

```console
Usage:
    target/debug/millipede [OPTIONS]

Print a beautiful millipede (written in rust)

optional arguments:
  -h,--help             show this help message and exit
  -w,--width WIDTH      millipede width
  -s,--skin SKIN        millipede skin
  -r,--reverse          reverse the millipede
```

## Example

```console
$> cargo run  -- --width 5
    ╚⊙ ⊙╝
  ╚═(███)═╝
 ╚═(███)═╝
╚═(███)═╝
 ╚═(███)═╝
  ╚═(███)═╝

$> cargo run  -- --width 5 --reverse
   ╔═(███)═╗
  ╔═(███)═╗
 ╔═(███)═╗
╔═(███)═╗
 ╔═(███)═╗
   ╔⊙ ⊙╗

$> cargo run  -- --width 5 --reverse --skin "OOO"
   ╔═(OOO)═╗
  ╔═(OOO)═╗
 ╔═(OOO)═╗
╔═(OOO)═╗
 ╔═(OOO)═╗
   ╔⊙ ⊙╗
```
