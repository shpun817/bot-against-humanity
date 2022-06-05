class LogDisplayError extends Error {
    constructor(displayMsg, logMsg = displayMsg) {
        super(logMsg);

        this.logMsg = logMsg;
        this.displayMsg = displayMsg;
    }
}

module.exports = { LogDisplayError };
