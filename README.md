# status-bot
A Rust script to check website status and notify on Discord if the site is down.

## Usage
Compile the program with `cargo build --release` ([installation](https://www.rust-lang.org/tools/install)). This will
create an executable file `target/release/status-bot` which you may then move to a better location.

Create a `.env` file to the same location where you have placed the `status-bot` executable. The `.env` file should
contain a webhook url and the url to test:
```shell
WEBHOOK_URL=<webhook url>
WEBSITE_URL=https://www.example.com
```

To run the website status check periodically (on macOS or Linux) I recommend using cron. You may edit the cron
configuration with `crontab -e`. I personally run the check every five minutes so I have the following line in the
configuration:
```shell
*/5 * * * * cd </path/to/folder> && ./status-bot
```

> **Note**  
> When just testing this program you should probably run it with `cargo run` instead of creating an executable.
