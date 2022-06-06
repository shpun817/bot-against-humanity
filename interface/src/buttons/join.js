const { createPrepareMsgOptions } = require("../commands/prepare");

module.exports = {
    name: "join",
    async handle(interaction) {
        // Format: join_<ownerId>
        const buttonId = interaction.customId;
        const ownerId = buttonId.split("_")[1];

        const user = interaction.user;
        const userMention = user.toString();

        const builder =
            interaction.client.gameInstanceManager.getBuilder(ownerId);
        const metadata =
            interaction.client.gameInstanceManager.getBuilderMetadata(ownerId);

        if (metadata.playerMentions.includes(userMention)) {
            await interaction.reply({
                content: "You have already joined this game!",
                ephemeral: true,
            });
            return;
        }

        builder.addPlayer(userMention);

        metadata.playerMentions.push(userMention);
        await metadata.prepareMsg.edit(
            createPrepareMsgOptions(ownerId, metadata.playerMentions),
        );
    },
};
