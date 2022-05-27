# Bot Against Humanity

This project provides a Discord bot that simulates [Cards Against Humanity](https://www.cardsagainsthumanity.com/), a party card game with simple rules:

1. Only 2 decks of cards: Question cards and Answer cards.
   - Question cards each contains either a question, or a statement with one or more *blanks* (denoted by  "__").
   - Answer cards each contains a word, a phrase, or even a sentence.
2. On each turn,
   1. Each player draws up to 10 Answer cards.
   2. One player is chosen as the **judge**.
   3. A Question card is drawn and shown to everyone.
   4. Except the judge, each player chooses Answer cards in their hand to **answer the question or fill in the blanks** on the Question card.
   5. Each set of cards played is revealed one by one. Enjoy the laugh from hilarious answers.
   6. **Without knowing** which set was played by who, the judge picks a personal favorite.
   7. The player who played the picked set gets one Awesome Point (AP).
3. When a player reaches, say, 5 AP, he/she wins the game.

(reference: [UltraBoardGames - Cards Against Humanity Game Rules](https://www.ultraboardgames.com/cards-against-humanity/game-rules.php#:~:text=Game%20Play,down%2C%20to%20the%20Card%20Czar.), some interesting house rules can also be found there)

## Systems Design

There will be 2 main systems interacting with each other in the project: `Core` and `Interface`, which are responsible for the *core game logic* and the *interface to the users*, respectively.

### Core

The `Core` system manages the game state.

The important classes are as follows:

Cards:

- `QuestionCard` has a question represented as a vector of tokens. A token is either a string or a blank.
- `AnswerCard` has an answer represented simply as a string.
- `CardStorage` has a deck of cards and a discard pile, both represented as a vector.

Agents:

- `Player` has an AP count represented as an integer and a hand represented as a vector of `AnswerCard`, the size of the hand vector should always be 10.
- `GameStateManager` has a vector of `Player` and two `CardStorage`, one for `QuestionCard` and one for `AnswerCard`. 

Exposed API:

- `GameLogicManager` coordinates how the game is run at any given time. It receives input from and sends back output to `Interface` to progress the game. It also handles player-id mapping.

### Interface

The `Interface` system handles raw input from the user, performs the **minimum amount of processing**, and feeds it to `Core` to progress the game logic. It is also responsible for displaying output to the user.

- `DiscordBot` is the main class in this event-driven system.
