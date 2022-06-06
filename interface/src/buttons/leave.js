const { createPrepareMsgOptions } = require("../commands/prepare");

module.exports = {
    name: "leave",
    async handle(interaction) {
        // Format: leave_<ownerId>
        const buttonId = interaction.customId;
        const ownerId = buttonId.split("_")[1];

        const user = interaction.user;
        const userId = user.id;
        const userMention = user.toString();

        const builder =
            interaction.client.gameInstanceManager.getBuilder(ownerId);
        const metadata =
            interaction.client.gameInstanceManager.getBuilderMetadata(ownerId);

        if (ownerId === userId) {
            await metadata.prepareInteraction.editReply({
                content: `${userMention} has cancelled the game. Use \`/prepare\` to start a new one!`,
                components: [],
            });

            interaction.client.gameInstanceManager.removeBuilder(ownerId);

            await interaction.reply({
                content: "Left successfully!",
                ephemeral: true,
            });

            return;
        }

        if (!metadata.playerMentions.includes(userMention)) {
            await interaction.reply({
                content: "You are not in this game!",
                ephemeral: true,
            });
            return;
        }

        builder.removePlayer(userMention);

        await interaction.reply({
            content: "Left successfully!",
            ephemeral: true,
        });

        metadata.playerMentions = metadata.playerMentions.filter(
            (p) => p !== userMention,
        );
        await metadata.prepareInteraction.editReply(
            createPrepareMsgOptions(ownerId, metadata.playerMentions),
        );
    },
};
