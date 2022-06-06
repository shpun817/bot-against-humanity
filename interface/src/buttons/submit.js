const { MessageActionRow, MessageButton } = require("discord.js");

function createDisplaySubmissionMessageOptions(
    submitResult,
    i,
    enableNext,
    enableChoose,
) {
    const content = `❓: **${submitResult[i][1]}**`;
    const components = [
        new MessageActionRow().addComponents(
            new MessageButton()
                .setCustomId(`next_${i}`)
                .setLabel("Next")
                .setStyle("PRIMARY")
                .setDisabled(!enableNext),
            new MessageButton()
                .setCustomId(`choose_${i}`)
                .setLabel("Choose")
                .setStyle("SUCCESS")
                .setDisabled(!enableChoose),
        ),
    ];

    return { content, components };
}

module.exports = {
    name: "submit",
    async handle(interaction) {
        // Format: submit
        const channelId = interaction.channel.id;

        const user = interaction.user;
        const userMention = user.toString();

        const driver =
            interaction.client.gameInstanceManager.getDriver(channelId);
        const metadata =
            interaction.client.gameInstanceManager.getDriverMetadata(channelId);

        // Only keeps the reference to the first time answers are submitted to avoid spamming
        if (!(userMention in metadata.playerSubmitInteractions)) {
            metadata.playerSubmitInteractions[userMention] = interaction;
        }
        const submitInteraction =
            metadata.playerSubmitInteractions[userMention];

        const currentSelectionIndices = metadata.playerSelections[userMention];

        let submitResult;
        try {
            submitResult = await driver.submitAnswers(
                userMention,
                currentSelectionIndices,
            );
        } catch (error) {
            const option = {
                content: error,
                ephemeral: true,
            };

            try {
                await submitInteraction.reply(option);
            } catch (_) {
                await submitInteraction.editReply(option);
            }

            return;
        }

        // The hand and answer interactions must have been assigned before the submit window showed up.
        const handInteraction = metadata.playerHandInteractions[userMention];
        await handInteraction.editReply({
            content: "Thanks for submitting your answers!",
            components: [],
        });
        const answerInteraction =
            metadata.playerAnswerInteractions[userMention];
        await answerInteraction.editReply({
            components: [],
        });

        const option = {
            content: `${userMention} is ready!`,
        };
        await interaction.reply(option);

        if (submitResult !== null) {
            await metadata.roundStartMessage.edit({ components: [] });

            await interaction.channel.send(
                "All players have submitted their answers!",
            );

            metadata.submitResult = submitResult;

            const displaySubmissionMessage = await interaction.channel.send(
                createDisplaySubmissionMessageOptions(
                    metadata.submitResult,
                    0,
                    true,
                    false,
                ),
            );

            metadata.submitResult[0].push(displaySubmissionMessage);

            return;
        }
    },
    createDisplaySubmissionMessageOptions,
};
