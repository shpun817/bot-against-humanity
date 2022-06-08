const { formatHand } = require("./hand");

module.exports = {
    name: "redraw",
    async handle(interaction) {
        // Format: redraw
        const channelId = interaction.channel.id;

        const user = interaction.user;
        const userMention = user.toString();

        const metadata =
            interaction.client.gameInstanceManager.getDriverMetadata(channelId);

        let content = "You will **NOT** redraw after this turn.";
        if (!metadata.redrawPlayers.has(userMention)) {
            metadata.redrawPlayers.add(userMention);

            content = "You **WILL** redraw after this turn.";
        } else {
            metadata.redrawPlayers.delete(userMention);
        }

        const hand = metadata.playerHands[userMention];
        const handInteraction =
            metadata.playerHandInteractions[userMention];
        const currentSelectionIndices =
            metadata.playerSelections[userMention];
        // Refresh the selections display
        await handInteraction.editReply(
            formatHand(
                currentSelectionIndices,
                hand,
                metadata.numBlanks,
                metadata.redrawPlayers.has(userMention),
            ),
        );

        await interaction.reply({ content, ephemeral: true });
    },
};
