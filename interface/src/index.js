require("dotenv").config();
const fs = require("node:fs");
const path = require("node:path");
const { Client, Collection, Intents } = require("discord.js");
const { BOT_TOKEN: token } = process.env;
const GameInstanceManager = require("./game_instance_manager");
const { LogDisplayError } = require("./error");

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

client.gameInstanceManager = new GameInstanceManager();

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
        if (error instanceof LogDisplayError) {
            console.error(error);
            await interaction.reply({
                content: error.displayMsg,
                ephemeral: true,
            });
        } else if (error instanceof Error) {
            console.error(error);
            await interaction.reply({
                content: "There was an error executing the command!",
                ephemeral: true,
            });
        }
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
