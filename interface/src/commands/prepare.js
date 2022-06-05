const { SlashCommandBuilder } = require("@discordjs/builders");
const { MessageActionRow, MessageButton } = require("discord.js");
const AssetLoader = require("../asset_loader");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("prepare")
        .setDescription("Prepare a new game!"),
    async execute(interaction) {
        const owner = interaction.user;
        const ownerId = owner.id;
        const ownerName = owner.username;
        const ownerMention = owner.toString();

        const gameInstanceManager = interaction.client.gameInstanceManager;
        gameInstanceManager.registerUsername(ownerId, ownerName);

        const builder = gameInstanceManager.createBuilder(ownerId);
        builder.addPlayer(ownerMention);
        builder.addNewQuestions(AssetLoader.LoadQuestionsJson("default"));
        builder.addNewAnswers(AssetLoader.LoadAnswersJson("default"));

        const components = [
            new MessageActionRow().addComponents(
                new MessageButton()
                    .setCustomId(`join_${ownerId}`)
                    .setLabel("Join")
                    .setStyle("SUCCESS"),
                new MessageButton()
                    .setCustomId(`leave_${ownerId}`)
                    .setLabel("Leave")
                    .setStyle("DANGER"),
            ),
        ];

        await interaction.reply({
            content: `${ownerMention} has started a game of Bot Against Humanity!`,
            components,
        });
    },
};
