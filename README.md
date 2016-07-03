# elevator

Simulating an elevator.

## Design

Elevator displays with floor buttons on the left. Elevator buttons for each floor on the right.

There are N floors. Floors are numbered from bottom to top with labels 1 to N. The lit up button is the current floor. That button ALWAYS remains disabled.

The floor buttons on the elevator display remain disabled until that elevator reaches a floor it was called to using the elevator buttons on the right.

Once the elevator reaches a floor, its floor buttons (except the current floor) become enabled. Those buttons can be used to select a floor. This times out after some number of seconds just like in a real elevator at which point the elevator continues to the next floor. The elevator buttons can become available again by pressing the directional buttons on the same floor (or by calling the elevator to another floor).

When idle, the elevator(s) should return to a floor where it would be most optimal to reach the other floors in a timely manner.

Eventually it should be possible to add elevators and floors in realtime and still have everything work. (Removing everything except for one elevator and one floor should also eventaully be added)

### ElevatorController struct

```rust
struct ElevatorController {
    elevators: Vec<Elevator>,
    floors: Vec<FloorControls>,
}
```

`floors` has at least one element. The first element is the ground floor.
The ground floor is also known as floor 1. Every floor added after that
is a higher floor above the ground floor.

This representation technically allows for basement floors too. Just
represent the lowest floor as floor 1. After all, you can display the
numbering however you want.

Elevators have a fixed velocity. Velocity is measured in `floors/tick`.
A `tick` is a call to the elevator's `update()` method. So a velocity of `1.0 floors/tick` is one floor per update. That can be useful when debugging.

All elevators are assumed to travel at the same velocity and the velocity
of elevators is assumed to always be constant.

### Elevator struct

```rust
struct Elevator {
    current_floor: Floor,
    target_floor: Option<Floor>,
    direction: Option<Direction>,
    // value between 1 and 100 where 100 = 1.0 floors/tick
    velocity: Floor,
}
```

When `direction` is `None` and `target_floor` is `None`, the elevator is considered idle.

`velocity` is `Floor` because it is measured in `floors/tick`.

### FloorControls struct

```rust
struct FloorControls {
    up: bool,
    down: bool,
}
```

### Floor newtype

```rust
struct Floor(u32);
```

Floors are in denominations of 100. This newtype enforces that and provides
methods to increase or decrease floors.

### Direction enum

```rust
enum Direction {
    Up,
    Down,
}
```

