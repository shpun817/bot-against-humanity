const fs = require("node:fs").promises;
const path = require("node:path");

const envPath = process.argv[2] ?? path.join(__dirname, "..");

const dotEnvTemplate =
    "BOT_TOKEN=<INSERT_YOUR_BOT_TOKEN>\n\
CLIENT_ID=<OPTIONAL_INSERT_YOUR_BOT_APPLICATION_ID>\n\
GUILD_ID=<OPTIONAL_INSERT_YOUR_DEV_SERVER_ID>\n\
\n\
GOOGLE_APPLICATION_CREDENTIALS=<OPTIONAL_INSERT_YOUR_GSHEET_SERVICE_ACCOUNT_SECRETS_PATH_RELATIVE_TO_PROJECT_ROOT>\n\
LIBRARY_SHEET_ID=<OPTIONAL_INSERT_YOUR_GSHEET_ID_FOUND_IN_URL>\n";

async function main() {
    if ((await fs.readdir(envPath)).includes(".env")) {
        throw `.env already exists at ${envPath}`;
    }

    await fs.writeFile(path.join(envPath, ".env"), dotEnvTemplate, {
        flag: "w",
    });
}

main().catch(console.error);
