const { WasmDriverBuilder } = require("bot-against-humanity-core");
const {
    generateMockAnswers,
    generateMockPlayers,
    generateMockQuestions,
} = require("./mock_generation");

describe("WasmDriverBuilder", () => {
    let builder;

    beforeEach(() => {
        builder = new WasmDriverBuilder();
    });

    describe("setting hand size", () => {
        test("can set the hand size.", () => {
            expect(() => builder.setHandSize(5)).not.toThrow();
        });

        test("cannot set a negative hand size.", () => {
            expect(() => builder.setHandSize(-2)).toThrow();
        });

        test("cannot set hand size to be zero.", () => {
            expect(() => builder.setHandSize(0)).toThrow();
        });
    });

    describe("adding players", () => {
        test("can add a player.", () => {
            expect(() => builder.addPlayer("A")).not.toThrow();
            expect(() => builder.addPlayer("B")).not.toThrow();
        });

        test("cannot add the same player twice.", () => {
            expect(() => builder.addPlayer("A")).not.toThrow();
            expect(() => builder.addPlayer("A")).toThrow();
        });
    });

    describe("removing players", () => {
        beforeEach(() => {
            builder.addPlayer("A");
            builder.addPlayer("B");
        });

        test("can remove a player.", () => {
            expect(() => builder.removePlayer("A")).not.toThrow();
        });

        test("cannot remove a non-existent player.", () => {
            expect(() => builder.removePlayer("C")).toThrow();
        });

        test("can remove all players.", () => {
            expect(() => builder.removeAllPlayers()).not.toThrow();
        });
    });

    test("can add and clear questions.", () => {
        expect(() =>
            builder.addNewQuestions([
                "Who am I?",
                "Kill __ and eat _.",
                "She is _.",
            ]),
        ).not.toThrow();

        expect(() => builder.clearAllQuestions()).not.toThrow();
    });

    test("can add and clear answers.", () => {
        expect(() =>
            builder.addNewAnswers([
                "Johnny",
                "Jesus Christ",
                "Mother Nature",
                "End of the World",
            ]),
        ).not.toThrow();

        expect(() => builder.clearAllAnswers()).not.toThrow();
    });

    describe("building a game driver", () => {
        const build = () => builder.build();
        const addPlayers = (n) => {
            generateMockPlayers(n).forEach((player) => {
                builder.addPlayer(player);
            });
        };
        const addQuestions = (n) => {
            builder.addNewQuestions(generateMockQuestions(n));
        };
        const addAnswers = (n) => {
            builder.addNewAnswers(generateMockAnswers(n));
        };

        test("can build a game driver.", () => {
            addPlayers(5);
            addQuestions(10);
            addAnswers(60);

            expect(build).not.toThrow();
        });

        test("cannot build without 3 players", () => {
            addQuestions(10);
            addAnswers(60);

            expect(build).toThrow();

            builder.addPlayer("A");
            expect(build).toThrow();

            builder.addPlayer("B");
            expect(build).toThrow();

            builder.addPlayer("C");
            expect(build).not.toThrow();
        });

        test("cannot build without any questions", () => {
            addPlayers(5);
            addAnswers(60);

            expect(build).toThrow();

            addQuestions(1);
            expect(build).not.toThrow();
        });

        test("cannot build without enough answers", () => {
            addPlayers(5);
            addQuestions(10);
            builder.setHandSize(10); // Hence 5*10 = 50 answers are required.

            addAnswers(49);
            expect(build).toThrow();

            addAnswers(1); // This "A1" is not unique in builder. It is not added as a new answer.
            expect(build).toThrow();

            addAnswers(50);
            expect(build).not.toThrow();
        });
    });
});
