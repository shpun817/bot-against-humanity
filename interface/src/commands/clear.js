const { SlashCommandBuilder } = require("@discordjs/builders");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("clear")
        .setDescription("Clear all questions or answers!")
        .addStringOption((option) =>
            option
                .setName("type")
                .setDescription("What to clear?")
                .setRequired(true)
                .addChoices(
                    { name: "questions", value: "questions" },
                    { name: "answers", value: "answers" },
                ),
        ),
    async execute(interaction) {
        await interaction.deferReply();

        const userId = interaction.user.id;
        const builder =
            interaction.client.gameInstanceManager.getBuilder(userId);

        const type = interaction.options.getString("type");
        switch (type) {
            case "questions":
                builder.clearAllQuestions();
                break;
            case "answers":
                builder.clearAllAnswers();
                break;
            default:
                throw "Unreachable: Unknown fetch `type`.";
        }

        await interaction.editReply(`Successfully cleared ALL ${type}!`);
    },
};
