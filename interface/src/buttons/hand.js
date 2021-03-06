const { MessageActionRow, MessageButton } = require("discord.js");

// `hand`: an array of strings
function formatHand(
    currentSelectionIndices,
    hand,
    numBlanks,
    isRedrawPressed = false,
) {
    const cardButtons = hand.map((card, i) =>
        new MessageButton()
            .setCustomId(`answer_${i}`)
            .setLabel(card)
            .setStyle(
                currentSelectionIndices.includes(i) ? "SUCCESS" : "PRIMARY",
            ),
    );

    const components = [];
    // Discord.js guide -> Buttons
    // You can have a maximum of five ActionRows per message, and five buttons within an ActionRow.
    // The last row is reserved for the `redraw` button, hence maximum 20 hand buttons.
    for (let i = 0; i < cardButtons.length && i < 20; ++i) {
        if (i % 5 === 0) {
            components.push(new MessageActionRow());
        }
        components[components.length - 1].addComponents(cardButtons[i]);
    }

    const redrawButton = new MessageButton()
        .setCustomId("redraw")
        .setLabel("Redraw AFTER this turn")
        .setStyle("DANGER");

    if (!isRedrawPressed) {
        redrawButton.setStyle("SECONDARY");
    }

    components.push(new MessageActionRow().addComponents(redrawButton));

    return {
        content: `*Ignore any errors you see in this menu.*\nSelect **${numBlanks}**.`,
        components,
        ephemeral: true,
    };
}

module.exports = {
    name: "hand",
    async handle(interaction) {
        // Format: hand
        const channelId = interaction.channel.id;

        const user = interaction.user;
        const userMention = user.toString();

        const metadata =
            interaction.client.gameInstanceManager.getDriverMetadata(channelId);

        if (userMention === metadata.judge) {
            throw "You are the Judge! Sit back and relax!";
        }

        if (!(userMention in metadata.playerHands)) {
            throw "You are not part of this game!";
        }

        if (metadata.successfullySubmittedPlayers.has(userMention)) {
            throw "You have already submitted an answer!";
        }

        const hand = metadata.playerHands[userMention];
        metadata.playerSelections[userMention] = [];
        metadata.playerHandInteractions[userMention] = interaction;

        await interaction.reply(formatHand([], hand, metadata.numBlanks));
    },
    formatHand,
};
