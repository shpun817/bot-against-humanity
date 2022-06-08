const { WasmDriverBuilder } = require("bot-against-humanity-core");
const {
    generateMockAnswers,
    generateMockPlayers,
    generateMockQuestions,
} = require("./mock_generation");

describe("WasmDriver", () => {
    const NUM_PLAYERS = 5;
    const HAND_SIZE = 8;

    let driver;
    let players;

    const questions = generateMockQuestions(10);
    const answers = generateMockAnswers(60);

    // Prepare a working driver
    beforeEach(() => {
        const builder = new WasmDriverBuilder();
        builder.setHandSize(HAND_SIZE);

        generateMockPlayers(NUM_PLAYERS).forEach((p) => builder.addPlayer(p));
        builder.addNewQuestions(questions);
        builder.addNewAnswers(answers);

        driver = builder.build();
        players = driver.orderedPlayers();
    });

    test("has the correct players.", () => {
        expect(players.length).toBe(NUM_PLAYERS);

        expect(players).toEqual(
            expect.arrayContaining(generateMockPlayers(NUM_PLAYERS)),
        );
    });

    test("can run a game.", () => {
        const { judge, question, playerHands } = driver.startRound();

        expect(players).toContain(judge);
        expect(questions).toContain(question.split(" ")[0]);

        expect(Object.keys(playerHands).length).toBe(NUM_PLAYERS);
        expect(Object.keys(playerHands)).toEqual(
            expect.arrayContaining(players),
        );

        for (const hand of Object.values(playerHands)) {
            expect(hand.length).toBe(HAND_SIZE);
            expect(answers).toEqual(expect.arrayContaining(hand));
        }

        expect(() => driver.submitAnswers(judge, [0])).toThrow();
        const nonJudgePlayers = players.filter((p) => p !== judge);
        expect(() => driver.submitAnswers("You-Know-Who", [0])).toThrow();
        expect(() => driver.submitAnswers(nonJudgePlayers[0], [-1])).toThrow();
        expect(() =>
            driver.submitAnswers(nonJudgePlayers[0], [HAND_SIZE]),
        ).toThrow();
        expect(() =>
            driver.submitAnswers(nonJudgePlayers[0], [0, 1]),
        ).toThrow();

        expect(() => {
            const submitResult = driver.submitAnswers(nonJudgePlayers[0], [0]);
            expect(submitResult).toBeNull();
        }).not.toThrow();
        expect(() => driver.submitAnswers(nonJudgePlayers[0], [0])).toThrow();

        for (let i = 1; i < nonJudgePlayers.length - 1; ++i) {
            expect(() => {
                const submitResult = driver.submitAnswers(nonJudgePlayers[i], [
                    i,
                ]);
                expect(submitResult).toBeNull();
            }).not.toThrow();
        }

        expect(() => {
            const submitResult = driver.submitAnswers(
                nonJudgePlayers[nonJudgePlayers.length - 1],
                [0],
            );
            expect(submitResult).not.toBeNull();
            expect(submitResult.length).toBe(nonJudgePlayers.length);
            for (const submittedAnswer of submitResult) {
                expect(submittedAnswer.length).toBe(2);

                const [playerName, answer] = submittedAnswer;
                expect(players).toContain(playerName);
                expect(typeof answer).toBe("string");
            }
        }).not.toThrow();

        expect(() =>
            driver.redrawHands([nonJudgePlayers[1], nonJudgePlayers[0]]),
        ).not.toThrow();

        expect(() => driver.endRound("You-Know-Who")).toThrow();
        expect(() => driver.endRound(judge)).toThrow();
        expect(() => driver.endRound(nonJudgePlayers[0])).not.toThrow();

        expect(() => driver.endGame()).not.toThrow();
    });
});
