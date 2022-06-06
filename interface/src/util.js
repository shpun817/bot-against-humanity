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

    content += `**${question.replaceAll("_", "\\_")}**`;

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

    metadata.playerHands = playerHands;
    metadata.playerSelections = {};
    metadata.playerHandInteractions = {};
    metadata.playerAnswerInteractions = {};
}

// `hand`: an array of strings
function formatHand(currentSelectionIndices, hand) {
    const cardButtons = hand.map((card, i) =>
        new MessageButton()
            .setCustomId(`answer_${i}`)
            .setLabel(card)
            .setStyle(currentSelectionIndices.includes(i) ? "SUCCESS" : "PRIMARY"),
    );

    const components = [];
    // Discord.js guide -> Buttons
    // You can have a maximum of five ActionRows per message, and five buttons within an ActionRow.
    for (let i = 0; i < cardButtons.length && i < 25; ++i) {
        if (i % 5 === 0) {
            components.push(new MessageActionRow());
        }
        components[components.length - 1].addComponents(cardButtons[i]);
    }

    return {
        content: "*Ignore any errors you see in this menu.*",
        components,
        ephemeral: true,
    };
}

module.exports = { formatHand, startRound };
