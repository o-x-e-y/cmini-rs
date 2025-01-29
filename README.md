# Cmini-rs

[Cmini](https://github.com/Apsu/cmini) but blazingly fast :fire: :100: :rocket:

No parity at all yet, RIIR in progress.

to run, make sure you have rust installed, yada yada. Then add a .env file:

```dotenv
APPLICATION_ID = // discord application ID
PUBLIC_KEY = // discord public key
DISCORD_TOKEN = // discord token
BOT_PERMISSIONS = 563224831454272

DATABASE_URL = sqlite://test.db
```

If that's all correct, you can run the following command:

```sh
cargo r -p rmini-discord
```
