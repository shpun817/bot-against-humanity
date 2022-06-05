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
            "You don't have a game instance being prepared, use `/prepare` to start preparing one!",
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
};

class GameInstanceManager {
    constructor() {
        this.ownerIdToBuilder = new Map();
        this.channelIdToDriver = new Map();
        this.userIdToUsername = new Map();
    }

    registerUsername(userId, displayName) {
        this.userIdToUsername.set(userId, displayName);
    }

    getUsername(userId) {
        if (!this.userIdToUsername.has(userId)) {
            throw errors.noRegisteredUsername(userId);
        }

        return this.userIdToUsername.get(userId);
    }

    createBuilder(ownerId) {
        if (this.ownerIdToBuilder.has(ownerId)) {
            throw errors.gameInstanceAlreadyBeingBuilt();
        }

        this.ownerIdToBuilder.set(ownerId, new WasmDriverBuilder());
        return this.ownerIdToBuilder.get(ownerId);
    }

    getBuilder(ownerId) {
        if (!this.ownerIdToBuilder.has(ownerId)) {
            throw errors.noGameInstanceBeingBuilt();
        }

        return this.ownerIdToBuilder.get(ownerId);
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
}

module.exports = GameInstanceManager;
