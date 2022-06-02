function generateMockPlayers(n) {
    return generateMockStrings("Player ", n);
}

function generateMockQuestions(n) {
    return generateMockStrings("Q", n);
}

function generateMockAnswers(n) {
    return generateMockStrings("A", n);
}

function generateMockStrings(prefix, n) {
    let strings = [];
    for (let i = 1; i <= n; ++i) {
        strings.push(prefix + i);
    }
    return strings;
}

module.exports = {
    generateMockAnswers,
    generateMockPlayers,
    generateMockQuestions,
};
