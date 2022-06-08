const { SlashCommandBuilder } = require("@discordjs/builders");
const AssetLoader = require("../asset_loader");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("fetch")
        .setDescription("Fetch data from GSheets!")
        .addStringOption((option) =>
            option
                .setName("type")
                .setDescription("What to fetch?")
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

        const libraryName = interaction.options.getString("library") ?? "default";

        let reply = "Fetching";
        switch (interaction.options.getString("type")) {
            case "questions":
                reply = await AssetLoader.FetchQuestions(libraryName);
                break;
            case "answers":
                reply = await AssetLoader.FetchAnswers(libraryName);
                break;
            default:
                throw "Unreachable: Unknown fetch `type`";
        }

        await interaction.editReply(reply);
    },
};
