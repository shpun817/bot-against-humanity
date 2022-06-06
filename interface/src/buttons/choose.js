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
            return "";
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

        await interaction.reply(`${chosenPlayerMention}, you are chosen!!`);

        const topScore = ranking[0][1];

        let rankingString = "";
        let lastScore = Infinity;
        let rank = 0;
        const topPlayers = [];

        for (const [playerMention, score] of ranking) {
            if (score < lastScore) {
                lastScore = score;
                rank += 1;
            }

            if (rank === 1) {
                topPlayers.push(playerMention);
            }

            rankingString += `${formatRank(rank)} ${playerMention} - **${score}**\n`;
        }

        await channel.send(rankingString);

        if (topScore >= metadata.winTarget) {
            const roundEndContent = `🎂🎉 ${topPlayers[0]} has won a ticket to hell!!! 🍾🎊`;

            await channel.send(roundEndContent);
            return;
        } else {
            let roundEndContent = `🔥 ${topPlayers[0]} is leading!! 🔥`;
            if (topPlayers.length > 1) {
                roundEndContent = `🔥🔥 ${topPlayers.reduce(
                    (acc, p, i, arr) => {
                        acc += p;
                        if (i < arr.length - 2) {
                            acc += ", ";
                        } else if (i === arr.length - 2) {
                            acc += " and ";
                        }
                        return acc;
                    },
                    "",
                )} are leading!! 🔥🔥`;
            }

            await interaction.followUp(roundEndContent);
        }

        await sleep(3000);

        await startRound(driver, channel, metadata);
    },
};
