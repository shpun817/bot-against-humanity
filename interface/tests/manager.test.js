/* eslint-disable */
const GameInstanceManager = require("../src/game_instance_manager");
const { WasmDriverBuilder, WasmDriver } = require("bot-against-humanity-core");
const {
    generateMockAnswers,
    generateMockPlayers,
    generateMockQuestions,
} = require("./mock_generation");

describe("GameInstanceManager", () => {
    let gameInstanceManager;

    beforeEach(() => {
        gameInstanceManager = new GameInstanceManager();
    });

    test("can create a builder if not exists.", () => {
        const builder = gameInstanceManager.createBuilder("A");

        expect(builder).toBeInstanceOf(WasmDriverBuilder);
    });

    test("cannot create a builder if already exists.", () => {
        gameInstanceManager.createBuilder("A");

        expect(() => gameInstanceManager.createBuilder("A")).toThrow();
    });

    test("can get a builder if already exists.", () => {
        gameInstanceManager.createBuilder("A");
        const builder1 = gameInstanceManager.getBuilder("A");

        expect(builder1).toBeInstanceOf(WasmDriverBuilder);

        const builder2 = gameInstanceManager.getBuilder("A");

        expect(builder2).toBeInstanceOf(WasmDriverBuilder);

        expect(builder1).toBe(builder2); // Same object
    });

    test("can remove a builder", () => {
        gameInstanceManager.createBuilder("A");

        expect(() => gameInstanceManager.removeBuilder("A")).not.toThrow();
        expect(() => gameInstanceManager.getBuilder("A")).toThrow();
    });

    test("can create a different builder for different owners.", () => {
        const builderA = gameInstanceManager.createBuilder("A");
        const builderB = gameInstanceManager.createBuilder("B");

        expect(builderA).not.toBe(builderB); // Different objects
    });

    describe("driver interactions", () => {
        beforeEach(() => {
            gameInstanceManager = new GameInstanceManager();
            const builder = gameInstanceManager.createBuilder("A");

            generateMockPlayers(3).forEach((p) => builder.addPlayer(p));
            builder.addNewQuestions(generateMockQuestions(10));
            builder.addNewAnswers(generateMockAnswers(50));
        });

        test("can build a driver.", () => {
            const driver = gameInstanceManager.buildDriver("A", "channelA");

            expect(driver).toBeInstanceOf(WasmDriver);
        });

        test("can no longer access the builder after it is used to build a driver.", () => {
            gameInstanceManager.buildDriver("A", "channelA");

            expect(() => gameInstanceManager.getBuilder("A")).toThrow();
        });

        test("cannot build a driver when there is already one in the thread.", () => {
            gameInstanceManager.buildDriver("A", "channelA");

            expect(() =>
                gameInstanceManager.buildDriver("A", "channelA"),
            ).toThrow();
        });

        test("can get a driver.", () => {
            const driver1 = gameInstanceManager.buildDriver("A", "channelA");
            gameInstanceManager.insertDriver("channelA", driver1);
            const driver2 = gameInstanceManager.getDriver("channelA");

            expect(driver1).toBe(driver2);
        });

        test("cannot get a driver before building it.", () => {
            expect(() => gameInstanceManager.getDriver("channelA")).toThrow();
        });

        test("can remove a driver", () => {
            const driver = gameInstanceManager.buildDriver("A", "channelA");
            gameInstanceManager.insertDriver("channelA", driver);

            expect(() =>
                gameInstanceManager.removeDriver("channelA"),
            ).not.toThrow();
        });

        test("cannot remove a driver before building it.", () => {
            expect(() =>
                gameInstanceManager.removeDriver("channelA"),
            ).toThrow();
        });
    });

    describe("submitting answers", () => {
        beforeEach(() => {
            gameInstanceManager = new GameInstanceManager();
        });

        test("can set submitted answers.", () => {
            gameInstanceManager.setSubmittedAnswers("channelA", [
                ["A", "I want to be Batman."],
            ]);
        });

        test("can get submitted answers.", () => {
            gameInstanceManager.setSubmittedAnswers("channelA", [
                ["A", "I want to be Batman."],
            ]);

            expect(gameInstanceManager.getSubmittedAnswers("channelA")).toEqual(
                [["A", "I want to be Batman."]],
            );
        });

        test("cannot get submitted answers before setting it.", () => {
            expect(() =>
                gameInstanceManager.getSubmittedAnswers("channelA"),
            ).toThrow();
        });

        test("can remove submitted answers.", () => {
            gameInstanceManager.setSubmittedAnswers("channelA", [
                ["A", "I want to be Batman."],
            ]);

            expect(() =>
                gameInstanceManager.removeSubmittedAnswers("channelA"),
            ).not.toThrow();
        });

        test("cannot remove submitted answers before setting it.", () => {
            expect(() =>
                gameInstanceManager.removeSubmittedAnswers("channelA"),
            ).toThrow();
        });
    });
});
