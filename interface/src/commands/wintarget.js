const { SlashCommandBuilder } = require("@discordjs/builders");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("wintarget")
        .setDescription("Set the win target!")
        .addIntegerOption((option) =>
            option.setName("value").setDescription(">= 1").setRequired(true),
        ),
    async execute(interaction) {
        const userId = interaction.user.id;
        const metadata =
            interaction.client.gameInstanceManager.getBuilderMetadata(userId);

        const winTarget = interaction.options.getInteger("value");

        if (winTarget < 1) {
            throw `Cannot set win target to ${winTarget}. It has to be at least 1.`;
        }

        metadata.winTarget = winTarget;

        await interaction.reply(`Successfully set win target to ${winTarget}`);
    },
};
