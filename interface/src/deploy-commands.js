require("dotenv").config();
const fs = require("node:fs");
const path = require("node:path");
const { REST } = require("@discordjs/rest");
const { Routes } = require("discord-api-types/v9");
const {
    CLIENT_ID: clientId,
    GUILD_ID: guildId,
    BOT_TOKEN: token,
} = process.env;

const commandsPath = path.join(__dirname, "commands");

const commands = fs
    .readdirSync(commandsPath)
    .filter((file) => file.endsWith(".js"))
    .map((file) => {
        const filePath = path.join(commandsPath, file);
        const command = require(filePath);
        return command.data.toJSON();
    });

const rest = new REST({ version: "9" }).setToken(token);

(async () => {
    try {
        console.log("Started refreshing application (/) commands.");

        await rest.put(Routes.applicationGuildCommands(clientId, guildId), {
            body: commands,
        });

        console.log("Successfully reloaded application (/) commands.");
    } catch (error) {
        console.error(error);
    }
})();
