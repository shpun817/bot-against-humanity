const { formatHand } = require("../util");

module.exports = {
    name: "hand",
    async handle(interaction) {
        // Format: hand
        const channelId = interaction.channel.id;

        const user = interaction.user;
        const userMention = user.toString();

        const metadata =
            interaction.client.gameInstanceManager.getDriverMetadata(channelId);

        if (!(userMention in metadata.playerHands)) {
            await interaction.reply({
                content: "You are not part of this game!",
                ephemeral: true,
            });
            return;
        }

        const hand = metadata.playerHands[userMention];
        metadata.playerSelections[userMention] = [];
        metadata.playerHandInteractions[userMention] = interaction;

        await interaction.reply(formatHand([], hand));
    },
};
