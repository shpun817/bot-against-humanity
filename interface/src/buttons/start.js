const { startRound } = require("../util");

module.exports = {
    name: "start",
    async handle(interaction) {
        // Format: start_<ownerId>
        const buttonId = interaction.customId;
        const ownerId = buttonId.split("_")[1];

        const user = interaction.user;
        const userId = user.id;
        const userName = user.username;

        if (userId !== ownerId) {
            await interaction.reply({
                content: "Only the owner of the game is allowed to start it!",
                ephemeral: true,
            });

            return;
        }

        const builderMetadata =
            interaction.client.gameInstanceManager.getBuilderMetadata(ownerId);

        const driver =
            interaction.client.gameInstanceManager.buildDriver(ownerId);

        const thread = await interaction.channel.threads.create({
            name: `${userName}'s Bot Against Humanity game (${new Date().toLocaleString()})`,
            autoArchiveDuration: 60,
            reason: "All aboard for a nice trip Against Humanity!",
        });

        await interaction.reply({
            content: "Game started successfully!",
            ephemeral: true,
        });
        await builderMetadata.prepareInteraction.editReply({
            content: `Players: ${
                builderMetadata.playerMentions
            }\nGame Started at ${thread.toString()}!`,
            components: [],
        });
        interaction.client.gameInstanceManager.insertDriver(thread.id, driver);
        const driverMetadata = interaction.client.gameInstanceManager.getDriverMetadata(thread.id);

        await startRound(driver, thread, driverMetadata);
    },
};
