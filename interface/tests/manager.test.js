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

    test("can register a user display name.", () => {
        gameInstanceManager.registerUsername("A", "Player A");
    });

    test("can get a registered user display name.", () => {
        gameInstanceManager.registerUsername("A", "Player A");

        expect(gameInstanceManager.getUsername("A")).toBe("Player A");
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
            gameInstanceManager.buildDriver("A", "channelA");
            const driver = gameInstanceManager.getDriver("channelA");

            expect(driver).toBeInstanceOf(WasmDriver);
        });

        test("cannot get a driver before building it.", () => {
            expect(() => gameInstanceManager.getDriver("channelA")).toThrow();
        });

        test("can remove a driver", () => {
            gameInstanceManager.buildDriver("A", "channelA");

            expect(() => gameInstanceManager.removeDriver("channelA")).not.toThrow();
        });

        test("cannot remove a driver before building it.", () => {
            expect(() =>
                gameInstanceManager.removeDriver("channelA"),
            ).toThrow();
        });
        
    });
});
