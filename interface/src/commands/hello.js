const { SlashCommandBuilder } = require("@discordjs/builders");
const { MessageActionRow, MessageButton } = require("discord.js");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("hello")
        .setDescription("Say hello to our friendly bot!"),
    async execute(interaction) {
        const components = [
            new MessageActionRow()
                .addComponents(
                    new MessageButton()
                        .setCustomId("hello")
                        .setLabel("Hello")
                        .setStyle("SUCCESS"),
                ),
        ];

        await interaction.reply({ content: "Hey yo!", components });
    },
};
