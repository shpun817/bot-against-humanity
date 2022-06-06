require("dotenv").config();
const fs = require("node:fs");
const path = require("node:path");
const { Client, Collection, Intents } = require("discord.js");
const { BOT_TOKEN: token } = process.env;
const GameInstanceManager = require("./game_instance_manager");
const { LogDisplayError } = require("./error");

async function handleError(error, interaction) {
    const options = { content: "Error", ephemeral: true };

    if (error instanceof LogDisplayError) {
        console.error(error);
        options.content = error.displayMsg;
    } else if (error instanceof Error) {
        console.error(error);
        options.content = `Error: ${error.message}`;
    } else if (typeof error === "string") {
        console.error(error);
        options.content = error;
    } else {
        console.error(error);
        options.content = "Unknown error!";
    }

    try {
        await interaction.reply(options);
    } catch (_) {
        await interaction.editReply(options);
    }
}

const client = new Client({
    intents: [Intents.FLAGS.GUILDS],
});

client.gameInstanceManager = new GameInstanceManager();

client.once("ready", () => {
    console.log("Ready!");
});

// Listen for commands
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
        await handleError(error, interaction);
    }
});

// Listen for button interactions
client.buttons = new Collection();
const buttonsPath = path.join(__dirname, "buttons");
fs.readdirSync(buttonsPath).forEach((file) => {
    if (!file.endsWith(".js")) {
        return;
    }
    const filePath = path.join(buttonsPath, file);
    const button = require(filePath);
    client.buttons.set(button.name, button);
});
client.on("interactionCreate", async (interaction) => {
    if (!interaction.isButton()) return;

    const buttonName = interaction.customId.split("_")[0];
    const buttons = interaction.client.buttons;

    if (!buttons.has(buttonName)) {
        console.error(
            `${interaction.user.tag} triggered an unknown button interaction with customId ${interaction.customId}`,
        );
        await interaction.reply({
            content: "Oops, you've pressed an unknown button!",
            ephemeral: true,
        });
        return;
    }

    try {
        await buttons.get(buttonName).handle(interaction);
    } catch (error) {
        await handleError(error, interaction);
    }
});

client.login(token);
