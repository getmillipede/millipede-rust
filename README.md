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

### OSX instructions

```console
$> brew install openssl
$> export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
$> export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
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
