const path = require("node:path");

class AssetLoader {
    static LoadQuestionsJson(libraryName) {
        const loadPath = path.join(AssetLoader.LocalBasePath, "questions", libraryName + ".json");
        return require(loadPath);
    }

    static LoadAnswersJson(libraryName) {
        const loadPath = path.join(AssetLoader.LocalBasePath, "answers", libraryName + ".json");
        return require(loadPath);
    }
}

AssetLoader.LocalBasePath = path.join(__dirname, "..", "assets");

module.exports = AssetLoader;
