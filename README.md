![Build Status](https://travis-ci.org/chasb96/drive.svg?branch=master)

# Project

A simple storage system plus more for personal use.

## Setup:

### Requirements:

* Postgres Database
* Rust: [installation instructions](https://www.rust-lang.org/en-US/install.html)

### Setup:

1. Create a Database in Postgres

2. Setup the `.env` file. Setup instructions can be found below

3. Setup Rocket.toml (Optional) Setup instructions can be found [here](https://rocket.rs/guide/configuration/#rockettoml)

5. `Diesel` needs the schema to match the project before it starts. `diesel migration run` is the [easiest](http://diesel.rs/guides/getting-started/) way to do this.

### Startup:
Simply run the command `cargo run` from your terminal. Any configuration options are specified in the `.env` file.

### Build Errors:
This project requires a package `Diesel` and `Rocket`. Before compiling, both `Diesel` and `Rocket` run other code to build dependencies.

This is important because if it cannot connect to the database, or it mismatches the database, it will look like a compiling error: thats because it is. When errors arise; the first place to look is your database.

As well, this project only works on `nightly` builds. If dependencies fail to build, ensure you're using the `nightly` build channel.

### ENV Options:

- Security
    - `APP_KEY`: Required, `string`. Security key used by the application to encrypt tokens.

    - `BCRYPT_COST`: Optional, `uint`. Number of hash iterations for passwords. Lower is faster but less secure, higher is slower but more secure. Default is `12`.

- Database
    - `DATABASE_URL`: Required, `string`. Connection string to your database. Looks something like `postgres://postgres:secret@db/database`.

    - `TEST_DATABASE_URL`: Required if running tests, `string`. Connection string to your testing database.

- Logging!
    - `LOG_LEVEL`: Optional, `string`. Minimum level at which logging should occur. Default is `error`.

- Storage
    - `STORAGE_DRIVER`: Optional, `string`. Driver that should be used to store objects, either `disk` or `aws`. Default is `disk`.

    - AWS
        - `AWS_ACCESS_KEY_ID`: Required with driver `aws`, `string`. Your AWS Access Key ID.

        - `AWS_ACCESS_KEY_SECRET`: Required with driver `aws`, `string`. Your AWS Access Key Secret.

        - `AWS_BUCKET_NAME`: Required with driver `aws`, `string`. Bucket to be used to store objects.

        - `AWS_BUCKET_REGION`: Required with driver `aws`, `string`. Bucket region to be used to store objects.

- Other
    - `STREAM_CHUNK_SIZE`: Optional, `string`. Size of the chunks to be used when streaming objects. Default is `1024`.
    > Note: The Linux kernel uses 256 by default, and this will not be overrode. Any chunk sizes smaller than this will have no impact
