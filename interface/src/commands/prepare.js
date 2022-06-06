const { SlashCommandBuilder } = require("@discordjs/builders");
const { MessageActionRow, MessageButton } = require("discord.js");
const AssetLoader = require("../asset_loader");

function createPrepareMsgOptions(ownerId, playerMentions) {
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

    return {
        content: `Players: ${playerMentions}`,
        components,
    };
}

module.exports = {
    data: new SlashCommandBuilder()
        .setName("prepare")
        .setDescription("Prepare a new game!"),
    async execute(interaction) {
        const owner = interaction.user;
        const ownerId = owner.id;
        const ownerMention = owner.toString();

        const gameInstanceManager = interaction.client.gameInstanceManager;

        const builder = gameInstanceManager.createBuilder(ownerId);
        const metadata = gameInstanceManager.getBuilderMetadata(ownerId);
        builder.addPlayer(ownerMention);
        builder.addNewQuestions(AssetLoader.LoadQuestionsJson("default"));
        builder.addNewAnswers(AssetLoader.LoadAnswersJson("default"));

        metadata.playerMentions = [ownerMention];
        metadata.prepareInteraction = interaction;

        await interaction.reply(
            createPrepareMsgOptions(ownerId, metadata.playerMentions),
        );
    },
    createPrepareMsgOptions,
};
