const path = require("node:path");

class AssetLoader {
    static LoadQuestionsJson(filename) {
        const loadPath = path.join(AssetLoader.BasePath, "questions", filename + ".json");
        return require(loadPath);
    }

    static LoadAnswersJson(filename) {
        const loadPath = path.join(AssetLoader.BasePath, "answers", filename + ".json");
        return require(loadPath);
    }
}

AssetLoader.BasePath = path.join(__dirname, "..", "assets");

module.exports = AssetLoader;
