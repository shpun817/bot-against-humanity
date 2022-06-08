const { SlashCommandBuilder } = require("@discordjs/builders");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("handsize")
        .setDescription("Set the hand size!")
        .addIntegerOption((option) =>
            option
                .setName("value")
                .setDescription("5-25, inclusive")
                .setRequired(true),
        ),
    async execute(interaction) {
        const userId = interaction.user.id;
        const builder =
            interaction.client.gameInstanceManager.getBuilder(userId);

        const handSize = interaction.options.getInteger("value");

        if (handSize < 5 || handSize > 25) {
            throw `Cannot set hand size to ${handSize}. It has to be between 5 and 25 inclusive.`;
        }

        builder.setHandSize(handSize);

        await interaction.reply(`Successfully set hand size to ${handSize}`);
    },
};
