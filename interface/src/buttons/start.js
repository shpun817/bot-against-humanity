const { MessageActionRow, MessageButton } = require("discord.js");

function formatPlayers(players, judge) {
    let content = "===================================\n";

    content += `Who is the **Judge** this turn? ${judge}!!\n`;

    for (const player of players) {
        content += judge === player ? `🧑‍⚖️ ${player} 👩‍⚖️` : player;
        content += "\n";
    }

    return content;
}

function formatQuestion(question) {
    let content = "===================================\n";

    content += "Question:\n\n";

    content += `**${question.replaceAll("_", "❓")}**`;

    const components = [
        new MessageActionRow().addComponents(
            new MessageButton()
                .setCustomId("hand")
                .setLabel("View Hand")
                .setStyle("PRIMARY"),
        ),
    ];

    return { content, components };
}

async function startRound(driver, channel, metadata) {
    const players = driver.orderedPlayers();
    const { judge, question, playerHands } = driver.startRound();

    await channel.send(formatPlayers(players, judge));

    await channel.send(formatQuestion(question));

    metadata.judge = judge;
    metadata.playerHands = playerHands;
    metadata.playerSelections = {};
    metadata.playerHandInteractions = {};
    metadata.playerAnswerInteractions = {};
    metadata.playerSubmitInteractions = {};
    metadata.submitResult = [];
}

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

        const thread = await interaction.channel.threads.create({
            name: `${userName}'s Bot Against Humanity game (${new Date().toLocaleString()})`,
            autoArchiveDuration: 60,
            reason: "All aboard for a nice trip Against Humanity!",
        });

        const driver = interaction.client.gameInstanceManager.buildDriver(
            ownerId,
            thread.id,
        );

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
        const driverMetadata =
            interaction.client.gameInstanceManager.getDriverMetadata(thread.id);

        await thread.send(`Get to **${driverMetadata.winTarget} Awesome Points** before anyone else!!`);
        await startRound(driver, thread, driverMetadata);
    },
    startRound,
};
