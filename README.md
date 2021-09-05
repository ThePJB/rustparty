probably just have a

MinigameManager
    - knows about all games
    - calls update etc
    - receives control signals like if something happened 
    - e.g. update returns a Signal like victory(red)
        - whoa its like control flow but in the other direction. Is that what IoC is about

- games could be dynamically dispatched via a trait: Update(input, dt), Draw and new
- and a way of selecting a random game

or

- could have an enum but then all defined together

------------------------------------------------------------------------------------------------------

input stuff is annoying 