const { createDisplaySubmissionMessageOptions } = require("./submit");

// Reference:
// submitResult is an array of arrays.
// Each element array is [playerMention, submittedAnswer, displaySubmissionMessage?].
// displaySubmissionMessage's up to the current submissionIndex one are assumed to have been pushed.
// i.e., the `next` buttons are pressed in sequence.

module.exports = {
    name: "next",
    async handle(interaction) {
        // Format: next_<submissionIndex>
        // For example, if `submissionIndex` is 0, it means the first (0th) `next` button is pressed.
        const submissionIndex = parseInt(interaction.customId.split("_")[1]);

        const channelId = interaction.channel.id;

        const user = interaction.user;
        const userMention = user.toString();

        const metadata =
            interaction.client.gameInstanceManager.getDriverMetadata(channelId);

        if (userMention !== metadata.judge) {
            throw "Listen to the **Judge**!";
        }

        const submitResult = metadata.submitResult;

        const currentDisplaySubmissionMessage =
            submitResult[submissionIndex][2];
        await currentDisplaySubmissionMessage.edit(
            createDisplaySubmissionMessageOptions(
                submitResult,
                submissionIndex,
                false,
                false,
            ),
        );

        const nextSubmissionIndex = submissionIndex + 1;
        const displaySubmissionMessage = await interaction.channel.send(
            createDisplaySubmissionMessageOptions(
                metadata.submitResult,
                nextSubmissionIndex,
                submissionIndex < submitResult.length - 2,
                false,
            ),
        );
        submitResult[nextSubmissionIndex].push(displaySubmissionMessage);

        if (submissionIndex === submitResult.length - 2) {
            await interaction.channel.send(
                "===================================\n" +
                    `That's all! Now **choose** your favourite, **Judge** ${userMention}!`,
            );
            // The end is reached. Set all previous messages accordingly.
            for (let i = 0; i < submitResult.length; ++i) {
                const message = submitResult[i][2];

                await message.edit(
                    createDisplaySubmissionMessageOptions(
                        submitResult,
                        i,
                        false,
                        true,
                    ),
                );
            }

            return;
        }
    },
};
