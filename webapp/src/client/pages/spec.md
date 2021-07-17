# Tournament Spec
## Introduction
Welcome to the first and highly experimental round of Marc Chee's Game AI Programming competition. This is not an official CSE competition, it's more of an experiment.

The goal of this competition is not to win, but to think about how we can improve our code in a series of challenges, both through fast iterations and sharing of code.

### Round structure
Round 1 starts July 5th. Round 1 ends July 24st. Round recap on Marc on Mondays stream July 26th.

### Communications
All competition communication will be done via the leaderboard website as well as the Marc on Mondays discord server: https://discord.gg/NDQnepT4N3

## The Game and Competition
The game for round 1 is a simplified version of a "Trick Taking" card game. If you want real world examples, you can look at Hearts, 500 and Bridge.

### Game rules

#### Players
This is a two player game.

#### Components
The initial deck of cards will consist of the Ace and numbers 2-5 of each of four suits. This is 20 cards.

#### Game Flow
The cards will be dealt randomly to the two players, 10 cards each.  
Players will be assigned the roles Player 1 and Player 2.  
Player 1 will start the game by playing a card.  
Each turn both players will play a card, making the game exactly 10 turns long.

#### A Single Game Turn
One player plays a card, which has a suit and a number (Aces are counted as 1).
- On the first turn, this is Player 1, but on subsequent turns it will be whoever won the last turn (also called a "trick")

The other player then must play a card.
- If the other player has card(s) that match the suit of the card that was played, they must play one of those cards. This is known as "following suit".
- If the other player doesn't have any cards that match the suit, they can play any cards
The highest card number in the suit chosen by the first player wins the "trick". That player scores one of the 10 available points and then gets to play first in the next round.

#### Game End
After 10 turns the game is over.  
Whoever has won more tricks is considered the winner.

### Competition Matchups
In competition, each match will consist of two symmetric games.  
This alleviates the randomness in the card deal.

The way the cards are dealt and the players' hands will be the same in both games, but the Player 1 and Player 2 roles will be swapped.

There will be 20 tricks in total in this matchup, split into the two games. Whoever wins the most tricks out of this 20 will be considered the winner of the matchup.

### Code
Players for this tournament must be written in C.  
There is sample starter code if you want to use it: TODO

Your code will be submitted to the competition and we will both compile it and run it against other players.

*Instructions for how to connect to the server and set up a profile*

There is also a test player that you can run against that will show you whether you are playing properly (basic legal play and correct response to the server).

*Instructions for playing immediately against the test player*

### Game Communications Details
All communications between your player and the game server will be done with simple standard input and output and entirely in integers.

Hopefully this makes it easy to get started (and our starter code will already have some of this set up).

#### Overview
Your program will receive information about the game, including which player you are, what cards are in your hand and the history of the game.

Your program will then output the identifying integers for a single card that is what you want to play in this situation.

Your program will not have to loop and run multiple turns, each run of the program is a single turn only.