const { SlashCommandBuilder } = require("@discordjs/builders");
const AssetLoader = require("../asset_loader");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("use")
        .setDescription("Clear all and then add questions or answers!")
        .addStringOption((option) =>
            option
                .setName("type")
                .setDescription("What to change?")
                .setRequired(true)
                .addChoices(
                    { name: "questions", value: "questions" },
                    { name: "answers", value: "answers" },
                ),
        )
        .addStringOption((option) =>
            option
                .setName("library")
                .setDescription("Name of the library/worksheet suffix")
                .setRequired(true),
        ),
    async execute(interaction) {
        await interaction.deferReply();

        const userId = interaction.user.id;
        const builder = interaction.client.gameInstanceManager.getBuilder(userId);

        const libraryName = interaction.options.getString("library");

        let reply = "Using";
        let data = [];
        switch (interaction.options.getString("type")) {
            case "questions":
                data = await AssetLoader.LoadQuestionsJson(libraryName);
                builder.clearAllQuestions();
                builder.addNewQuestions(data);
                reply = `Successfully added ${data.length} questions in ${libraryName}.`;
                break;
            case "answers":
                data = await AssetLoader.LoadAnswersJson(libraryName);
                builder.clearAllAnswers();
                builder.addNewAnswers(data);
                reply = `Successfully added ${data.length} questions in ${libraryName}.`;
                break;
            default:
                throw "Unreachable: Unknown use `type`.";
        }

        await interaction.editReply(reply);
    },
};
