const { startRound } = require("./start");

// Reference:
// submitResult is an array of arrays.
// Each element array is [playerMention, submittedAnswer, displaySubmissionMessage?].
// displaySubmissionMessage's up to the current submissionIndex one are assumed to have been pushed.
// i.e., the `next` buttons are pressed in sequence.

function formatRank(rank) {
    switch (rank) {
        case 1:
            return "🏆";
        default:
            return rank;
    }
}

function sleep(ms) {
    return new Promise((_) => setTimeout(_, ms));
}

module.exports = {
    name: "choose",
    async handle(interaction) {
        // Format: choose_<submissionIndex>
        // For example, if `submissionIndex` is 0, it means the first (0th) `choose` button is pressed.
        const submissionIndex = parseInt(interaction.customId.split("_")[1]);

        const channel = interaction.channel;
        const channelId = channel.id;

        const user = interaction.user;
        const userMention = user.toString();

        const driver =
            interaction.client.gameInstanceManager.getDriver(channelId);
        const metadata =
            interaction.client.gameInstanceManager.getDriverMetadata(channelId);

        if (userMention !== metadata.judge) {
            throw "Wait for your turn to be the **Judge**!!";
        }

        const submitResult = metadata.submitResult;
        const chosenPlayerMention = submitResult[submissionIndex][0];

        const ranking = driver.endRound(chosenPlayerMention);

        let rankingString = "";
        let lastScore = Infinity;
        let rank = 0;

        for (const [playerMention, score] of ranking) {
            if (score < lastScore) {
                lastScore = score;
                rank += 1;
            }

            rankingString += `${formatRank(rank)} ${playerMention}\n`;
        }

        await interaction.reply(rankingString);

        await sleep(3000);

        await startRound(driver, channel, metadata);
    },
};
