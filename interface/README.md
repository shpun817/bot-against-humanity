# Bot Against Humanity - Discord Bot Interface

## Setup for hosting

1. Fork or clone this repository.
2. Create a new Discord Bot at [Discord Developer Portal](https://discord.com/developers/applications).
   - Take note of the client id (application id) and the client secret (token) of your bot.
   - [Reference](https://discordjs.guide/preparations/setting-up-a-bot-application.html)
3. Run `npm install` and `npm run setup`. You should see a new (hidden) file named `.env`.
4. Paste the client secret into the `BOT_TOKEN` field in `.env`.
5. **Edit** the existing question-/answer- libraries (`default.json`), or **create** your own in `assets/questions` and `assets/answers`.
6. Run `npm start`. Everything should be working fine if you see "Ready" printed on the screen.
