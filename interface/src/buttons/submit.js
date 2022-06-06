const { MessageActionRow, MessageButton } = require("discord.js");

function createDisplaySubmissionMessageComponents(enableNext, enableChoose) {
    return [
        new MessageActionRow().addComponents(
            new MessageButton()
                .setCustomId("next_0")
                .setLabel("Next")
                .setStyle("PRIMARY")
                .setDisabled(!enableNext),
            new MessageButton()
                .setCustomId("choose_0")
                .setLabel("Choose")
                .setStyle("SUCCESS")
                .setDisabled(!enableChoose),
        ),
    ];
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

        const currentSelectionIndices = metadata.playerSelections[userMention];

        const option = {
            content: `${userMention} is ready!`,
            ephemeral: false,
        };

        try {
            const submitResult = await driver.submitAnswers(
                userMention,
                currentSelectionIndices,
            );

            if (submitResult !== null) {
                await interaction.reply(option);
                await interaction.channel.send(
                    "All players have submitted their answers!",
                );

                metadata.submitResult = submitResult;

                const displaySubmissionMessage = await interaction.channel.send(
                    {
                        content: `**${submitResult[0][1]}**`,
                        components: createDisplaySubmissionMessageComponents(
                            true,
                            false,
                        ),
                    },
                );

                metadata.submitResult[0].push(displaySubmissionMessage);

                return;
            }
        } catch (error) {
            option.content = error;
            option.ephemeral = true;
        }

        await interaction.reply(option);
    },
    createDisplaySubmissionMessageComponents,
};
