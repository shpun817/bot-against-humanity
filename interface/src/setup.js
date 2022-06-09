const fs = require("node:fs").promises;
const path = require("node:path");

const rootPath = process.argv[2] ?? path.join(__dirname, "..");

const dotEnvTemplate =
    "BOT_TOKEN=<INSERT_YOUR_BOT_TOKEN>\n\
CLIENT_ID=<OPTIONAL_INSERT_YOUR_BOT_APPLICATION_ID>\n\
GUILD_ID=<OPTIONAL_INSERT_YOUR_DEV_SERVER_ID>\n\
\n\
GOOGLE_APPLICATION_CREDENTIALS=<OPTIONAL_INSERT_YOUR_GSHEET_SERVICE_ACCOUNT_SECRETS_PATH_RELATIVE_TO_PROJECT_ROOT>\n\
LIBRARY_SHEET_ID=<OPTIONAL_INSERT_YOUR_GSHEET_ID_FOUND_IN_URL>\n";

const questionsTemplate =
    '[\n\
    "Who am I?",\n\
    "Among all the things in the world, my favourites are _, _, and _."\n\
]\n';

const answersTemplate =
    '[\n\
    "None of your business?",\n\
    "Superman",\n\
    "Batman",\n\
    "God",\n\
    "Satan"\n\
]\n';

async function main() {
    if ((await fs.readdir(rootPath)).includes(".env")) {
        throw `.env already exists at ${rootPath}`;
    }

    await fs.writeFile(path.join(rootPath, ".env"), dotEnvTemplate, {
        flag: "w",
    });

    const assetsPath = path.join(rootPath, "assets");
    const questionsPath = path.join(assetsPath, "questions");
    const answersPath = path.join(assetsPath, "answers");

    try {
        await fs.writeFile(
            path.join(questionsPath, "default.json"),
            questionsTemplate,
        );
    } catch (error) {
        console.error(error);
    }

    try {
        await fs.writeFile(
            path.join(answersPath, "default.json"),
            answersTemplate,
        );
    } catch (error) {
        console.error(error);
    }
}

main().catch(console.error);
