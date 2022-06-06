const { WasmDriverBuilder } = require("bot-against-humanity-core");
const { LogDisplayError } = require("./error");

const errors = {
    noRegisteredUsername: (userId) =>
        new Error(
            `The display name of the user with id ${userId} is not registered.`,
        ),
    gameInstanceAlreadyBeingBuilt: (ownerId) =>
        new LogDisplayError(
            "You already have a game instance being prepared!",
            `Owner with user id ${ownerId} already has a game instance being prepared.`,
        ),
    noGameInstanceBeingBuilt: (ownerId) =>
        new LogDisplayError(
            "The game instance is not being prepared, use `/prepare` to start preparing one!",
            `Owner with user id ${ownerId} does not have a game instance being prepared.`,
        ),
    threadAlreadyHasGameInstance: (channelId) =>
        new LogDisplayError(
            "Cannot reuse channel for game instance!",
            `Channel with id ${channelId} already has a game instance.`,
        ),
    noRunningGameInstance: (channelId) =>
        new LogDisplayError(
            "There is no running game instance in this channel!",
            `Channel with id ${channelId} does not have a running game instance.`,
        ),
    noSubmittedAnswers: (channelId) =>
        new Error(
            `Channel with id ${channelId} does not have any submitted answers.`,
        ),
};

class GameInstanceManager {
    constructor() {
        this.ownerIdToBuilder = new Map();
        this.channelIdToDriver = new Map();
        this.channelIdToSubmittedAnswers = new Map();
    }

    createBuilder(ownerId) {
        if (this.ownerIdToBuilder.has(ownerId)) {
            throw errors.gameInstanceAlreadyBeingBuilt();
        }

        this.ownerIdToBuilder.set(ownerId, {
            builder: new WasmDriverBuilder(),
            metadata: {},
        });
        return this.ownerIdToBuilder.get(ownerId).builder;
    }

    getBuilder(ownerId) {
        if (!this.ownerIdToBuilder.has(ownerId)) {
            throw errors.noGameInstanceBeingBuilt();
        }

        return this.ownerIdToBuilder.get(ownerId).builder;
    }

    getBuilderMetadata(ownerId) {
        if (!this.ownerIdToBuilder.has(ownerId)) {
            throw errors.noGameInstanceBeingBuilt();
        }

        return this.ownerIdToBuilder.get(ownerId).metadata;
    }

    removeBuilder(ownerId) {
        if (!this.ownerIdToBuilder.has(ownerId)) {
            throw errors.noGameInstanceBeingBuilt();
        }

        this.ownerIdToBuilder.delete(ownerId);
    }

    buildDriver(ownerId, channelId) {
        if (!this.ownerIdToBuilder.has(ownerId)) {
            throw errors.noGameInstanceBeingBuilt();
        }

        if (this.channelIdToDriver.has(channelId)) {
            throw errors.threadAlreadyHasGameInstance();
        }

        const driver = this.getBuilder(ownerId).build();
        this.ownerIdToBuilder.delete(ownerId);

        this.channelIdToDriver.set(channelId, driver);

        return driver;
    }

    getDriver(channelId) {
        if (!this.channelIdToDriver.has(channelId)) {
            throw errors.noRunningGameInstance(channelId);
        }

        return this.channelIdToDriver.get(channelId);
    }

    removeDriver(channelId) {
        if (!this.channelIdToDriver.has(channelId)) {
            throw errors.noRunningGameInstance(channelId);
        }

        this.channelIdToDriver.delete(channelId);
    }

    setSubmittedAnswers(channelId, answers) {
        this.channelIdToSubmittedAnswers.set(channelId, answers);
    }

    getSubmittedAnswers(channelId) {
        if (!this.channelIdToSubmittedAnswers.has(channelId)) {
            throw errors.noSubmittedAnswers(channelId);
        }

        return this.channelIdToSubmittedAnswers.get(channelId);
    }

    removeSubmittedAnswers(channelId) {
        if (!this.channelIdToSubmittedAnswers.has(channelId)) {
            throw errors.noSubmittedAnswers(channelId);
        }

        this.channelIdToSubmittedAnswers.delete(channelId);
    }
}

module.exports = GameInstanceManager;
