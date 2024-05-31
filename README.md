# How to run
Run the command `cargo run` from the repo directory and follow the instructions of the CLI.
Enter only valid card numbers from the original game.
# How are the best moves calculated
The solver keeps track of every card, that has been played so far. From this information it determines the number of unplayed cards between your hand cards and cards ending on stacks. If there is a stack with 5 cards, it calculates the number of possible cards in the interval set by other ending cards.
After calculating this information it can choose the best possible play for the given situation. This has a limitation of not being able to predict if keeping some cards might be beneficial for later rounds.
