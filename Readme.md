# keyword-data-migration

Here you will find a programmatic way of getting data from a microservice and store it on a database.
This repo is using [Rust language](https://www.rust-lang.org/).

## Where to start?

[Rust The Book](https://doc.rust-lang.org/book/) is where you can find more about the language.

[Getting started with Rust](https://www.rust-lang.org/learn/get-started) is where you can find how to install it.

## Tools used in this repo
[reqwest](https://github.com/seanmonstar/reqwest): HTTP Client for rust

[diesel](http://diesel.rs/): ORM and Query Builder for Rust

[mysqlclient](https://pypi.org/project/mysqlclient/): For using diesel with mysql, maybe you have to install this library.
For more about this, you can check [Getting Started with Diesel](http://diesel.rs/guides/getting-started/).

## Some basic introduction

With Rust language installation you will get [Cargo](https://github.com/rust-lang/cargo) which is the Rust dependency package manager.

#### Build package
```
$ cargo build
``` 

#### Build optimized package
```
$ cargo build --release
``` 

### How to execute it?
First things first...

This application needs .env and setting.toml files to execute. There we have some configurations so that application can run successfully.

#### .env file example
```properties
DATABASE_HOST="a-nice-host"
DATABASE_USER="a-nice-database-username"
DATABASE_PASSWORD="a-very-secure-password"
DATABASE_NAME="database-name"
```

#### .settings file example
```properties
keyword_url = "https://some-place-to-get-keywords-from"
keyword_id_start = 1
max_keyword_id = 20000
statistics_url = "https://some-place-to-get-keyword-ids-statistics-"
batch_size = 10000
```

#### Put both 2 files above in the same execution place
```bash
$ ./keyword-data-migration
``` 

#### You should start seeing something like output below
```bash
2020-01-24 01:31:56,675 INFO  [keyword_data_migration] Starting import
2020-01-24 01:32:04,818 INFO  [keyword_data_migration] Imported keywords from 1 to 9999 in 5867 milliseconds. Total execution in 8
2020-01-24 01:32:10,327 INFO  [keyword_data_migration] Imported keywords from 10000 to 20000 in 5508 milliseconds. Total execution in 13
2020-01-24 01:32:15,620 INFO  [keyword_data_migration] Total execution from 1 to 20000 in 18 seconds
```