const { SlashCommandBuilder } = require("@discordjs/builders");
const AssetLoader = require("../asset_loader");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("add")
        .setDescription("Add questions or answers!")
        .addStringOption((option) =>
            option
                .setName("type")
                .setDescription("What to add?")
                .setRequired(true)
                .addChoices(
                    { name: "questions", value: "questions" },
                    { name: "answers", value: "answers" },
                ),
        )
        .addStringOption((option) =>
            option
                .setName("optional_library")
                .setDescription("Name of the library/worksheet suffix"),
        ),
    async execute(interaction) {
        await interaction.deferReply();

        const userId = interaction.user.id;
        const builder =
            interaction.client.gameInstanceManager.getBuilder(userId);

        const libraryName =
            interaction.options.getString("optional_library") ?? "default";

        let reply = "Adding";
        let data = [];
        switch (interaction.options.getString("type")) {
            case "questions":
                data = await AssetLoader.LoadQuestionsJson(libraryName);
                builder.addNewQuestions(data);
                reply = `Successfully added ${data.length} questions in ${libraryName}.`;
                break;
            case "answers":
                data = await AssetLoader.LoadAnswersJson(libraryName);
                builder.addNewAnswers(data);
                reply = `Successfully added ${data.length} questions in ${libraryName}.`;
                break;
            default:
                throw "Unreachable: Unknown add `type`.";
        }

        await interaction.editReply(reply);
    },
};
