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

architecture:
    lets ECS it a bit its more modular
    so collision just needs array of all transforms
    is a transform just a rect?
    you can filter out before calling movement bounds. brah dop so gud

    gotta figure out how im going to handle adding / removing entities with the AoS method then
    probably actually have a HashMap per component
    add entity needs to add the relevant components
    remove entity needs to clear all components

    how to add entity while iterating tho? maybe the same way u do all mutation, side effect enums
    surely it would be easy af to have hooks for shit too that way. command pattern good

    strafe button

do something about a: maybe just start game centered on origin and dont require a

other minigame ideas:
    platofrming race where u can jump on each other, block etc. that would need good collision