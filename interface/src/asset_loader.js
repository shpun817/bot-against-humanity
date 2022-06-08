const fs = require("node:fs").promises;
const path = require("node:path");
const { google } = require("googleapis");
const { LogDisplayError } = require("./error");

class AssetLoader {
    static MakeQuestionsPath(libraryName) {
        return path.join(
            AssetLoader.LocalBasePath,
            "questions",
            libraryName + ".json",
        );
    }

    static MakeAnswersPath(libraryName) {
        return path.join(
            AssetLoader.LocalBasePath,
            "answers",
            libraryName + ".json",
        );
    }

    static LoadQuestionsJson(libraryName) {
        const loadPath = AssetLoader.MakeQuestionsPath(libraryName);
        return require(loadPath);
    }

    static LoadAnswersJson(libraryName) {
        const loadPath = AssetLoader.MakeAnswersPath(libraryName);
        return require(loadPath);
    }

    static async FetchQuestions(libraryName) {
        const worksheetName = "questions-" + libraryName;
        const questions = await fetchColumnA(worksheetName);
        const stringifiedQuestions = prettyStringify(questions);

        const writePath = AssetLoader.MakeQuestionsPath(libraryName);
        try {
            await fs.writeFile(writePath, stringifiedQuestions);
        } catch (error) {
            throw new LogDisplayError(
                `Error writing to ${writePath}`,
                error.message,
            );
        }

        return `Successfully fetched ${questions.length} questions into storage!`;
    }

    static async FetchAnswers(libraryName) {
        const worksheetName = "answers-" + libraryName;
        const answers = await fetchColumnA(worksheetName);
        const stringifiedAnswers = prettyStringify(answers);

        const writePath = AssetLoader.MakeAnswersPath(libraryName);
        try {
            await fs.writeFile(writePath, stringifiedAnswers);
        } catch (error) {
            throw new LogDisplayError(
                `Error writing to ${writePath}`,
                error.message,
            );
        }

        return `Successfully fetched ${answers.length} answers into storage!`;
    }
}

AssetLoader.LocalBasePath = path.join(__dirname, "..", "assets");

async function fetchSheets() {
    let auth;
    try {
        auth = await google.auth.getClient({
            keyFile: path.join(
                __dirname,
                "..",
                process.env.GOOGLE_APPLICATION_CREDENTIALS,
            ),
            scopes: ["https://www.googleapis.com/auth/spreadsheets.readonly"],
        });
    } catch (error) {
        throw new LogDisplayError(
            "Google Sheets API authentication failed!",
            error.message,
        );
    }

    try {
        return google.sheets({ version: "v4", auth });
    } catch (error) {
        throw new LogDisplayError(
            "Error fetching the Google Sheets resources!",
            error.message,
        );
    }
}

async function fetchColumnA(worksheetName) {
    const sheets = await fetchSheets();
    const range = `${worksheetName}!A:A`;

    let response;
    try {
        response = await sheets.spreadsheets.values.get({
            spreadsheetId: process.env.LIBRARY_SHEET_ID,
            range,
        });
    } catch (error) {
        throw new LogDisplayError(
            `Error fetching worksheet with name ${worksheetName}!`,
            error.message,
        );
    }

    const data = response.data.values.flat().filter(filterFetchedValue);

    return Array.from(new Set(data));
}

function filterFetchedValue(value) {
    // Null-ish values
    if (value === null || value === undefined || typeof value === "undefined") {
        return false;
    }

    // Whitespace only
    if (value.trim().length === 0) {
        return false;
    }

    return true;
}

function prettyStringify(value) {
    return JSON.stringify(value, null, 4);
}

module.exports = AssetLoader;
