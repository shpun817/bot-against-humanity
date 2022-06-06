const { MessageActionRow, MessageButton } = require("discord.js");

function formatPlayers(players, judge) {
    let content = "Take turns to be the Judge!\n";

    for (const player of players) {
        content += player;
        if (judge === player) {
            content += " 👩‍⚖️";
        }
        content += "\n";
    }

    return content;
}

function formatQuestion(judge, question) {
    let content = "===================================\n";

    content += `Show ${judge} what you got:\n\n`;

    content += `${question.replaceAll("_", "\\_")}\n\n`;

    const components = [
        new MessageActionRow().addComponents(
            new MessageButton()
                .setCustomId("hand")
                .setLabel("View my Hand")
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

    metadata.playerHands = playerHands;
}

module.exports = { startRound };
