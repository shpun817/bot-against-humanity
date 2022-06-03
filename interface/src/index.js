require("dotenv").config();
const fs = require("node:fs");
const path = require("node:path");
const { Client, Collection, Intents } = require("discord.js");
const { BOT_TOKEN: token } = process.env;

const client = new Client({
    intents: [Intents.FLAGS.GUILDS],
});
client.commands = new Collection();
const commandsPath = path.join(__dirname, "commands");
fs.readdirSync(commandsPath).forEach((file) => {
    if (!file.endsWith(".js")) {
        return;
    }
    const filePath = path.join(commandsPath, file);
    const command = require(filePath);
    client.commands.set(command.data.name, command);
});

client.once("ready", () => {
    console.log("Ready!");
});

// Listen for commands
client.on("interactionCreate", async (interaction) => {
    if (!interaction.isCommand()) return;

    const command = client.commands.get(interaction.commandName);

    if (!command) {
        console.error(`Unknown command: ${interaction.commandName}`);
        await interaction.reply({
            content: "This command is not available!",
            ephemeral: true,
        });
        return;
    }

    try {
        await command.execute(interaction);
    } catch (error) {
        console.error(error);
        await interaction.reply({
            content: "There was an error while executing this command!",
            ephemeral: true,
        });
    }
});

// Listen for button interactions
client.on("interactionCreate", async (interaction) => {
    if (!interaction.isButton()) return;

    await interaction.reply({
        content: "Button is pressed!",
    });
});

client.login(token);
