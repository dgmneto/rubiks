# rubiks

This is a rubik's cube solver that can be executed as a binary in a terminal. This repo also includes analysis of different approaches to find the solution of the rubik's cube: Dijkstra, A* with different score functions and BFS.

## Representation

We'll represent the rubik's cube faces with 3x3 arrays, having a 6x3x3 array to represent a whole cube. If we flatten the cube and represent the collors with numbers, this is what a solved cube looks like in our representation:

```
         face 0
        +-------+
        | 0 0 0 |
        | 0 0 0 |
 face 1 | 0 0 0 |
+-------+-------+-------+
| 1 1 1 | 2 2 2 | 3 3 3 |
| 1 1 1 | 2 2 2 | 3 3 3 | face 3
| 1 1 1 | 2 2 2 | 3 3 3 |
+-------+-------+-------+
        | 4 4 4 |
        | 4 4 4 | face 4
        | 4 4 4 |
        +-------+
        | 5 5 5 |
        | 5 5 5 | face 5
        | 5 5 5 |
        +-------+
```

The face in the middle is the face 2. Each face is store in the respective index of our array: face 0 in `array[0]`, face 1 in `array[1]` and so on.

## Movements (transformations)

[All possible movements](http://www.rubiksplace.com/move-notations/) on a real rubik's cube are represented as transformations to our array. For example, assuming face 2 is the front face, if we do a R rotation followed by a U' rotation, these would be the 2 new states:

```
             +-------+                     +-------+
             | 0 0 2 |                     | 2 2 2 |
             | 0 0 2 |                     | 0 0 0 |
             | 0 0 2 |                     | 0 0 0 |
     +-------+-------+-------+     +-------+-------+-------+
  R  | 1 1 1 | 2 2 4 | 3 3 3 |  U' | 0 5 5 | 1 1 1 | 2 2 4 |
 --> | 1 1 1 | 2 2 4 | 3 3 3 | --> | 1 1 1 | 2 2 4 | 3 3 3 |
     | 1 1 1 | 2 2 4 | 3 3 3 |     | 1 1 1 | 2 2 4 | 3 3 3 |
     +-------+-------+-------+     +-------+-------+-------+
             | 4 4 5 |                     | 4 4 5 |
             | 4 4 5 |                     | 4 4 5 |
             | 4 4 5 |                     | 4 4 5 |
             +-------+                     +-------+
             | 5 5 0 |                     | 5 5 0 |
             | 5 5 0 |                     | 5 5 0 |
             | 5 5 0 |                     | 3 3 3 |
             +-------+                     +-------+
```

All possible transformations are preprocessed when we start the application and stored as a map of the moves that are executed. Some of the moves in the U' tranformation are:

```
(1, 0, 0) -> (5, 2, 2)
(1, 0, 1) -> (5, 2, 1)
(1, 0, 2) -> (5, 2, 0)
...
(0, 0, 0) -> (0, 0, 2)
(0, 0, 1) -> (0, 1, 2)
(0, 0, 2) -> (0, 2, 2)
...
```

## Solution finding approaches

TODO: complete with all used approaches

## Backlog

This is what is left to be done ordered by priority:

- [X] Implement represantion of rubik's cube
- [ ] Preprocess transformations
- [ ] Implement Dijkstra
- [ ] Implement A* with some score function
- [ ] Implement BFS
- [ ] Compare approaches and amend README with findings
- [ ] Implement proper CLI
  - [ ] Define protocol of input and output
  - [ ] Define important options and arguments