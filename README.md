# rust-postgres
This project was just to learn postgresql with rust, i don't know what it is now.

## Installing Rust

MacOS, Linux, Windows (with a pm)
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


## Installing postgresql

MacOS
```bash
$ brew install postgresql
```
Arch
```bash
$ pacman -S postgresql
```
OpenSUSE
```bash
$ sudo zypper postgresql postgresql-server postgresql-contrib
```

Ubuntu
```bash
$ sudo apt-get install postgresql
```

Windows: https://www.postgresqltutorial.com/install-postgresql/

## Starting Posgresql

MacOS
```bash
$ brew services start postgresql
```

Linux
```bash
$ systemctl start postgresql.service
```
