
## Prerrequisitos

```
$ sudo yum install gcc-c++
$ sudo yum install wget ncurses-devel texinfo svn python-devel
```

## Compilar

```
 $ mkdir sources
 $ mkdir opt
 $ cd sources/
sources $ wget ftp://ftp.gnu.org/gnu/gdb/gdb-8.2.tar.gz
sources $ tar -xzvf gdb-8.2.tar.gz
sources $ mkdir gdb-8.2.build
sources $ cd gdb-8.2.build
gdb-8.2.build $ ../gdb-8.2/configure --with-python=yes --prefix=$HOME/opt
gdb-8.2.build $ make
```

Configurar environment:

PATH=$PATH:$HOME/.local/bin:$HOME/bin:**$HOME/opt/bin**

## Debugging Rust:

Ver rust_gdb.md



