const { MessageActionRow, MessageButton } = require("discord.js");
const { formatHand } = require("./hand");

module.exports = {
    name: "answer",
    async handle(interaction) {
        // Format: answer_<cardIndex>
        const cardIndex = parseInt(interaction.customId.split("_")[1]);

        const channelId = interaction.channel.id;

        const user = interaction.user;
        const userMention = user.toString();

        const metadata =
            interaction.client.gameInstanceManager.getDriverMetadata(channelId);

        // Only keeps the reference to the first time an answer is selected to avoid spamming
        if (!(userMention in metadata.playerAnswerInteractions)) {
            metadata.playerAnswerInteractions[userMention] = interaction;
        }

        const hand = metadata.playerHands[userMention];
        const handInteraction = metadata.playerHandInteractions[userMention];
        const currentSelectionIndices = metadata.playerSelections[userMention];

        const answerInteraction =
            metadata.playerAnswerInteractions[userMention];

        if (!currentSelectionIndices.includes(cardIndex)) {
            currentSelectionIndices.push(cardIndex);
        } else {
            const position = currentSelectionIndices.indexOf(cardIndex);
            currentSelectionIndices.splice(position, 1);
        }

        // Refresh the selections
        await handInteraction.editReply(
            formatHand(currentSelectionIndices, hand),
        );

        const currentSelectionWords = currentSelectionIndices
            .map((i) => hand[i])
            .reduce((acc, answer, i, arr) => {
                acc += `**${answer}**`;
                if (i !== arr.length - 1) {
                    acc += ", ";
                }
                return acc;
            }, "");

        const option = {
            content: `You selected ${currentSelectionWords}`,
            components: [
                new MessageActionRow().addComponents(
                    new MessageButton()
                        .setCustomId("submit")
                        .setLabel("Submit")
                        .setStyle("SUCCESS")
                        .setDisabled(currentSelectionIndices.length !== metadata.numBlanks),
                ),
            ],
            ephemeral: true,
        };

        try {
            await answerInteraction.reply(option);
        } catch (_) {
            await answerInteraction.editReply(option);
        }
    },
};
