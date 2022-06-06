const { MessageActionRow, MessageButton } = require("discord.js");

function formatPlayers(players, judge) {
    let content = "Take turns to be the Judge!\n";

    for (const player of players) {
        content += judge === player ? `🧑‍⚖️ ${player} 👩‍⚖️` : player;
        content += "\n";
    }

    return content;
}

function formatQuestion(judge, question) {
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

    await channel.send(formatQuestion(judge, question));

    metadata.judge = judge;
    metadata.playerHands = playerHands;
    metadata.playerSelections = {};
    metadata.playerHandInteractions = {};
    metadata.playerAnswerInteractions = {};
    metadata.playerSubmitInteractions = {};
    metadata.submitResult = [];
}

module.exports = { startRound };
