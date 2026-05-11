# Rubik's Cube Solver Part 2  

## May 2, 2026
I found an approach that works. My solver can now find a solution for a cube that was scrambled in 33 moves in 26ms on my laptop. I realized it just wasn't feasable to write a well performing solver with less than a megabyte of tables. I did some googling and found that I can use external flash to store and access larger tables, particularly for phase 2. The plan right now is to use a QSPI external flash device, particularly the Adafruit W25Q128. It is an external Quad-SPI flash memory chip that can provide up to 16MB of storage. The access latency at runtime will be around 350ns, compared to 10ns internal flash access, but it shouldn't be that noticeable. 

The problem is, my current MCU, the Nucleo-F401RE, doesn't have a dedicated QSPI peripheral, so for this approach, I would also need to get something like a Nucleo-F446RE. I might have to reconfigure the whole project from scratch on the CubeMX IOC for the new MCU, and then manually port over all the code and linker settings, but I'm not sure yet. I'll need to keep looking into that. 

Either way, I generated the large tables and have been able to run some tests.

| Scramble Moves | P1 Time(ms) | P1 Nodes | P1 Depth  | P2 Time(ms) | P2 Nodes  | P2 Depth |
| -------------- | ----------- | -------- | --------  | ----------- | --------  | -------- |
| 10             | 234         |  5,415    |    10    |  18         |  24,887   |    14    |
| 11             | 732         | 16,866    |    10    |  73         |  97,484   |    15    |
| 12             | 394         |  9,054    |    10    |   3         |  12,532   |    13    |
| 13             | 109         |  2,530    |    10    |  25         |  30,785   |    13    | 
| 14             |  52         |  1,200    |     9    |  24         |  29,455   |    13    |
| 15             |  28         |    655    |     9    |   2         |   2,879   |    13    |
| 16             |  23         |    538    |     9    |   2         |   2,762   |    13    |
| 17             | 222         |  5,202    |    10    |  49         |  62,167   |    15    |
| 18             | 130         |  3,016    |    10    | 147         | 179,396   |    15    |
| 19             | 133         |  3,113    |    10    |   1         |   4,562   |    12    |
| 20             |  25         |    604    |     9    |  82         |  95,389   |    15    |
| 21             | 181         |  4,221    |    10    |  23         |  28,666   |    13    |
| 22             |  28         |    639    |     9    |   1         |   2,491   |    12    |
| 23             | 379         |  8,851    |    10    |   1         |  10,703   |    12    |
| 24             | 532         | 12,421    |    10    |   4         |  16,215   |    12    |
| 25             | 577         | 13,428    |    10    |  27         |  43,061   |    13    |

As you can see, the solver is significantly faster than the previous implementations.  
Now we can do an actual EBF calculation, unlike the one I did a few days ago. 

$$
b^*=N^{(1/d)}
$$

where N is total nodes explored in the search and d is depth where solution was found. Of course, for Phase 1, the "solution" is just getting the cube in the G1 subgroup.  

We can find the EBF for both IDA* searches by calculating it for each n-moves scrambled cube test, after which taking the geometric mean of all of them would result in a EBF score for each phase. Remember, an EBF of 1 means the heuristic is perfect.

I won't show the whole calculation here, but for example, for the EFB for Phase 1, we would start with the 10 move scrambled cube test.

$$
b^*[10] = 5,415 ^ {(1/10)} = 2.19
$$

I would then do the same for the 11 move test, 12 move test, etc. Then, taking the geometric mean of each test's EBF (specifically the 16 tests 10-25),

$$
(\prod_{i=10}^{25}b^*[i] )^{(1/16)}
$$

We get 2.25 as the overall Effective Branching Factor for Phase 1. That means for every node explored, it expands on average to 2.25 more nodes. This means the algorithm is exploring a small fraction of the search space, and that the heuristic is efficient in cutting it down. We do the same computations over Phase 2's test results, and get a EBF of 2.13, indicating better performance than Phase 1, which makes sense due to the larger size and number of tables Phase 2 has access to. About that, below is the current setup for the tables:

Stored in Internal Flash (F446RE has 1MB): 
| Table               | Size (B)     | Purpose 
| -----               | ------------ | -------  
| CORNER_ORIENT_TABLE |  2,187       | Phase 1 heuristic
| CO_MOVE_TABLE       | 78,732       | Phase 1 search
| EO_MOVE_TABLE       | 73,728       | Phase 1 search
| UD_MOVE_TABLE       | 17,820       | Phase 1 search
| SP_MOVE_TABLE       |    240       | Phase 2 search
  
Total internal: 172,707 bytes

QSPI External Flash (W25Q128 = 16MB):
| Table               | Size (B)     | Purpose 
| -----               | ------------ | -------  
| FLIP_UDSLICE_TABLE   |      1,013,760 bytes | Phase 1 heuristic
| CORNERS_SLICE2_TABLE |        967,680 bytes | Phase 2 heuristic
| EDGES_SLICE2_TABLE   |        967,680 bytes | Phase 2 heuristic
| CP_MOVE_TABLE        |        806,400 bytes | Phase 2 search
| EP_MOVE_TABLE        |        806,400 bytes | Phase 2 search

Total QSPI: 4,561,920 bytes

Each phase has two heuristic tables and three search tables. The phase 1 tables are 1,186,227 bytes, and the phase 2 tables are 3,548,400 bytes. In total, I am using 4,734,627 bytes of tables. As for what each of them do:

### CORNER_ORIENT_TABLE
Size: 2,187 bytes  
What it stands for: Corner orientation pruning. Each of the 8 corners can be twisted 0, 1, or 2 times. The 8th corner's twist is always determined by the other 7 (physical invariant), so there are 3^7 = 2,187 possible states.  
What the value means: The minimum number of moves required to make all 8 corners untwisted, ignoring everything else about the cube.  
Why this size: One byte per state. 3^7 = 2,187 states. 2,187 × 1 byte = 2,187 bytes.  
When used: Phase 1 heuristic. Every node in the Phase 1 search does one lookup here and uses the result as part of the lower bound estimate.

### FLIP_UDSLICE_TABLE    
Size: 1,013,760 bytes ≈ 1MB  
What it stands for: Combined edge orientation + UD-slice membership pruning. This is the key Phase 1 table. It jointly encodes EO (2,048 states) and UD-slice membership (495 states).  
What the value means: The minimum moves to simultaneously achieve EO=0 AND UD=0.  
Why this size: 2,048 EO states × 495 UD states × 1 byte = 1,013,760 bytes.  
When used: Phase 1 heuristic, every node.  
Lives in: QSPI external flash.

### CORNERS_SLICE2_TABLE
Size: 967,680 bytes ≈ 945KB  
What it stands for: Combined corner permutation + belt permutation pruning. Jointly encodes CP (40,320 states) and SP (24 states).
What the value means: The minimum Phase 2 moves to simultaneously achieve CP=0 AND SP=0. 
Why this size: 40,320 CP states × 24 SP states × 1 byte = 967,680 bytes.  
When used: Phase 2 heuristic, every node.  
Lives in: QSPI external flash.

### EDGES_SLICE2_TABLE
Size: 967,680 bytes ≈ 945KB  
What it stands for: Combined non-belt edge permutation + belt permutation pruning. Same structure as CORNERS_SLICE2 but for the 8 non-belt edges instead of the 8 corners.  
What the value means: The minimum Phase 2 moves to simultaneously achieve EP=0 AND SP=0.  
Why this size: 40,320 EP states × 24 SP states × 1 byte = 967,680 bytes.  
When used: Phase 2 heuristic alongside CORNERS_SLICE2.  
Lives in: QSPI external flash.

### CO_MOVE_TABLE
Size: 78,732 bytes ≈ 77KB  
What it stands for: Corner orientation move transitions. For every CO coordinate value and every one of the 18 moves, stores the resulting CO coordinate.  
What the value means: CO_MOVE[co][mi] = the new corner orient coord after applying move mi to a cube with CO coord co.  
Why this size: 2,187 CO states × 18 moves × 2 bytes (u16, since values reach 2,186) = 78,732 bytes.  
When used: Phase 1 search, every node, every move tried.  
Lives in: Internal flash.

### EO_MOVE_TABLE
Size: 73,728 bytes ≈ 72KB  
What it stands for: Edge orientation move transitions. Same structure as CO_MOVE but for the 2,048 EO states.  
What the value means: EO_MOVE[eo][mi] = new edge orient coord after applying move mi.  
Why this size: 2,048 EO states × 18 moves × 2 bytes = 73,728 bytes.  
When used: Phase 1 search, every node.  
Lives in: Internal flash.

### UD_MOVE_TABLE
Size: 17,820 bytes ≈ 17KB  
What it stands for: UD-slice membership move transitions. For every UD coord and every move, stores the resulting UD coord.  
What the value means: UD_MOVE[ud][mi] = new UD-slice membership coord after applying move mi.  
Why this size: 495 UD states × 18 moves × 2 bytes = 17,820 bytes.  
When used: Phase 1 search, every node.  
Lives in: Internal flash.

### CP_MOVE_TABLE
Size: 806,400 bytes ≈ 787KB
What it stands for: Corner permutation move transitions. For every CP coord and every one of the 10 Phase 2 moves, stores the resulting CP coord.
What the value means: CP_MOVE[cp][mi] = new corner perm coord after applying Phase 2 move mi.
Why this size: 40,320 CP states × 10 Phase 2 moves × 2 bytes = 806,400 bytes.
When used: Phase 2 search, every node, every move tried.
Lives in: QSPI external flash.

### EP_MOVE_TABLE
Size: 806,400 bytes ≈ 787KB  
What it stands for: Non-belt edge permutation move transitions. Same structure as CP_MOVE but for the 40,320 EP states.  
What the value means: EP_MOVE[ep][mi] = new edge perm coord after applying Phase 2 move mi.  
Why this size: 40,320 EP states × 10 Phase 2 moves × 2 bytes = 806,400 bytes.  
When used: Phase 2 search, every node.  
Lives in: QSPI external flash.

### SP_MOVE_TABLE
Size: 240 bytes  
What it stands for: Belt (slice) permutation move transitions. For every SP coord and every Phase 2 move, stores the resulting SP coord.  
What the value means: SP_MOVE[sp][mi] = new belt perm coord after applying Phase 2 move mi.  
Why this size: 24 SP states × 10 Phase 2 moves × 1 byte (u8, values reach 23) = 240 bytes.  
When used: Phase 2 search, every node.  
Lives in: Internal flash.

I'll be doing more research into QSPI, as well as figuring out how to migrate my project between target MCUs on the IDE, but for now I'll just be happy that I now have a solver that can actually solve a cube in my lifetime!

## May 1, 2026
Well I tried the Kociemba approach I described below, and it seems its a lot faster than the single IDA* implementation, but still way too slow. Here's a performance ratio table:

| Moves | Phase 1 Time(s)  | Phase 2 Time(s)  |
| ----- | ---------------- | ---------------- |
| 10    | 32.17            |   390 (6.5 m)    |
| 11    | 62.10            |   2706 (45 m)    |
| 12    | 36.10            |   66.05          |
| 13    | 09.73            |   53.29          |
| 14    | 02.61            |   53.27          |
| 15    | 02.76            |   24.65          |
| 16    | 01.79            |   24.75          |
| 17    | 23.04            |   1705 (28 m)    |
| 18    | 13.55            |   2811 (47 m)    |
| 19    | 16.38            |   04.18          |
| 20    | 03.33            |   1879 (31 m)    |

Obviously this isn't a perfect measurement of performance but it can give you an idea of what's going on.

## April 27, 2026  
I wrote the IDA* implementation with four modified databases. Korf's full implementation used the following pattern databases for the heuristic:   

Corner PDB: position + orientation for all 8 corners -> 8! * 3^7 = 88MB  
Edge PDB 1: position + orientation for first 6 of 12 edges -> P(12, 6) * 2^6 = 43MB  
Edge PDB 2: position + orientation for other 6 of 12 edges -> 43MB  
Total: 174MB. The Nucleo only has 512KB of flash.

Instead of the canonical tables, I focused on the following state spaces:    
Corner Orientation: 2187 bytes (3 ways to orient 7 corners, 3^7 = 2,187)     
Corner Permutation: 40,320 bytes (all permutations of 8 corners: 8! = 40,320)     
Edge Orientation: 2048 bytes (2 ways to orient 11 edges: 2^11 = 2,048)  
UD Slice: 495 bytes (The number of ways to choose 4 slots out of 12 is C(12,4) = 495)     
Total: 44KB. I did away with edge position altogether, save for the UD-slice edges.   

The heuristic for the solver uses these tables to determine the lower bound solution length, and on my laptop, the solver works, kind of. It takes about 8 seconds on my 3.2GHz M1 Pro CPU to find a solution for a cube that was scrambled in 10 moves. The Nucleo has a 84MHz CPU, so:   
3.2GHz / 84MHz = 38.1x slowdown. Therefore, it would take:    
8 seconds * 38.1 = 4.75 minutes on MCU.    

Even on my laptop, though, the algorithm suffers when it is tasked with solving a cube was scrambled in 11 or more moves. Table below shows how long it takes to solve an n-moves scrambled cube. The ratio from 0-moves to 5-moves is so small it won't be considered here. The Effective Branching Factor (b*) is a measure of the efficiency of a heuristic search algorithm. It represents the average number of successor nodes a search algorithm expands, weighted to account for the effectiveness of the pruning. If a heuristic is perfect, b* is 1. The table below is not the right application of finding b*, but it provides useful information nonetheless, and it can give us an idea as to what the b* might be.

For the Ratio column, 

$$
ratio(n) = time(n) / time(n-1):
$$

| Moves | Time (ms) | Ratio |
| ----- | --------- | ----- |
| 0     | 0.010     |   -   |
| 1     | 0.096     |   -   |
| 2     | 0.081     |   -   |
| 3     | 0.123     |   -   |
| 4     | 0.116     |   -   |
| 5     | 0.430     |   -   |
| 6     | 4         |  9.3  |
| 7     | 16        |   4   |
| 8     | 360       | 22.5  |
| 9     | 797       |  2.2  |
| 10    | 8784      |  7.7  |

Taking the geometric mean of the last five ratios gives us our EFB

$$
b^* = (9.3 * 4.0 * 22.5 * 2.2 * 11.0) ^ {(1/5)} = 7.2
$$

So on average, every node visited expands to 7 succesor nodes. That's really not great. Either way, to predict forward, using the time taken to solve the 10-move scrambled cube, we just use the following equation:

$$
time(n) = 8,784 * b^{*(n-10) }
$$

To predict how long it would take an 12-move scrambled cube, for example:

$$
time(12) = 8,784 * 7.2^2 = 455,362ms \approx7.5~mins
$$

So then our final table looks like this (all values are approximations): 

| N-moves scrambled cube | Solve time on M1      | Solve time on MCU | 
| ----- | --------------- | ----------- |
|  10   |  8.8 s          |  5.5 mins   |
|  11   |  1 min          |  40 mins    |
|  12   |  7.5 mins       |  4.8 hours  |
|  13   |  1 hour         |  1.5 days   |
|  14   |  6.5 hours      |  10.5 days  |
|  15   |  2 days         |  75 days    |
|  16   |  14 days        |  1.5 years  |
|  17   |  102 days       |  10.5 years |
|  18   |  2 years        |  76.5 years |
|  19   |  14.5 years     |  5 centuries|
|  20   |  1 century      |  4 millenia |

Considering four things:
1. The number of moves it takes to scramble the cube is generally the same number of moves the solver generates.
2. God's number states that any cube can be solved in 20 or less moves. 
3. My weak heuristic almost entirely ensures that my solver will generate more than 20 moves for any sufficiently scrambled cube.
4. I don't know how to kill time for a 4 millenia.

That's a little more time than I'd like it to take, so the current implementation won't work. So, I'm switching gears and trying out a Kociemba 2-phase implementation:

Kociemba splits the solve into two sequential IDA* searches, each over a much smaller search space than trying to solve the whole cube at once:    

### Phase 1 
Reduces the cube to a special subgroup called G1. G1 is the set of all cube states reachable using only the 6 moves U, D, R2, L2, F2, B2. A cube is in G1 when:   
* All corners are correctly oriented (CO = 0)  <-- corner orientation  
* All edges are correctly oriented (EO = 0)    <-- edge orientation
* All UD-slice edges are in their belt (equatorial) slots (UD = 0)  <-- ud-slice combination

That's the first IDA* search

### Phase 2
Solves from G1 to solved, using only the 10 moves U, U2, Ui, D, D2, Di, R2, L2, F2, B2. Because the move set is restricted, the search space collapses dramatically.
* All corners are correctly positioned (CP = 0) <-- corner permutation
* All UD-slice edges are correctly positioned (UP = 0) <-- ud slice permutation
* All edges are correctly positioned (EP = 0) <-- edge permutation

So all I have to do to write Kociemba from here is just add two more tables and coordinate functions (the UD-Slice Permutation and Edge Permutation tables), write each phase's DFS function, the entry point function, surely some helper functions, and that should be it!

I already had four tables from my previous attempt, which total take up 44KB. Regarding the new tables' role and size:  

UD-Slice Permutation Table: The UD-Slice Combination table from Phase 1 ensures the four equatorial edges indeed make it to the four equator slots. The Edge Orientation Table ensures all 12 edges are unflipped, including the UD-Slice ones, so that's great. But Phase 2 needs to make sure they get to the right position. So the search space for this table is the four equatorial cubies, and the number of ways to permute that is 4! = 24, so this table will only be 24 bytes long.

Edge Permutation Table: The 8 remaining non UD-Slice edges need to be in the correct position. Again, thanks to the Edge Orientation table, we don't need to worry about flipping them. We just need to permute all 8 remaining edges for this table, which, just like the Corner Permutation table, needs 8! elements to cover all possible cube states. So another 40320 bytes

All together, the six tables should take up 85KB of flash. Again, the Nucleo has 512KB of available flash space, so I should be good. With decent heuristics, I should be able to lower the EFB from 7.2 to something more reasonable, like a 3 or 4. Let's see how it turns out.



## April 24, 2026       

I underestimated this project before. I won't repeat that same mistake again.           
Cards on the table, I still don't know how to solve a Rubik's Cube. But it turns out I might not have to in order to write this. I did some research, and I learned about an algorithm that I didn't pay much attention to when I was first taking a crack at this that sounds like it was designed for my exact case. I don't know how I missed it, but Wikipedia says that "solving the Rubik's Cube is an example of a planning problem that is amenable to solving with IDA*". Not only that, but the IDA* search algorithm "requires an amount of memory that is only linear in the length of the solution that it constructs", meaning that it is practically hand-made for embedded environments. So this project is my attempt at writing a modified iterative deepening A* algorithm in Rust no-std.      
I say modified because in a research paper written by Richard Korf (the original writer of IDA*), he describes an application of the algorithm specifically for solving a Rubik's Cube, which requires about 174MB of lookup tables (after pruning). I'm using an STM32 Nucleo-F401RE, which has only 512KB flash and 96KB RAM, so I have to modify the heuristic of the algorithm to be even more memory friendly. I definitely have the option to write the full, unmodified IDA* solver on a PC, which would theoretically generate a solution in less than a second, but I like the idea of the robot itself doing all the thinking. 
Included in the Documentation folder is the Korf research paper as well as my running notes for the project. I'll leave you with a rather germane message from Korf himself:    

"The problem is quite diffcult."

# Rubik's Cube Solver Part 1  
March 15, 2026  

Welcome to my Rubik's Cube Solver project!  

I started this project on January 16, 2026, a few weeks after I wrapped up my [Robotic Arm project](https://github.com/JorgeLarach/Robotic-Arm). I really wanted this to be my biggest project yet, and I wanted to focus more on the software side of things rather than the mechanical or design aspects, which I felt had been stealing the spotlight from my last couple of projects. The software for this project is structured in three parts:
1. Python GUI with Tkinter for transmitting initial cube state: This script runs on the host device, and it allows the user to color-in the 54 cube stickers according to their actual scrambled physical cube. There's a cube validation script that runs in the background before the 54 bytes are sent through pyserial to the MCU, which, for this project, is an STM32 Nucleo-F401RE.
2. Rust no-std static library solver: Once the cube configuration is in the MCU's RAM, the user can feel free to unplug the MCU from the host machine. The solver runs entirely on machine, and the user presses a button to execute it. A few caveats: the solver isn't actually written yet; that's part 2 of the project. Turns out its rather challenging to write a Rubik's cube solver with no-std, and much more so if you've never actually solved the thing on your own. That being said, learning to solve a cube is one of the main reasons why I chose this project. Furthermore, it certainly isn't necessary to have the solver run on-machine with no-std. This is an intentional, self imposed constraint. I wanted to practice writing an FFI between C and Rust, as the latter's prevalence in industry seems to be increasing year after year, and I also enjoy programming in Rust. As of now, the solver outputs a constant move list, regardless of the initial cube configuration, of which the second half of the list is the inverse of the first half. It basically scrambles and unscrambles a solved cube.

3. C motor driver with FreeRTOS: This module is responsible for receiving UART packets from the host machine, calling the Rust solver, and activating the motor hardware timer. It is functionally the code that ties the project together. It manages a cube_state struct, which stores the initial cube configuration, the move list output by the solver, a translated stepper_move list which the motor module uses to execute the moves, and a series of boolean flags to ensure no processes are interrupting each other or executing in an incorrect order.

This project uses the standard Rubik's cube move notation, where 'L' denotes a 90 degree clockwise turn of the Left face, 'Li' denotes a 90 degree counterclockwise turn, and 'L2' denotes a 180 degree turn. Across all three languages used in this project, I standardized the cube notation order and orientation as follows: Up - White, Down - Yellow, Left - Green, Right - Blue, Front - Red, and Back - Orange. For example, when the 54 byte UART packet is received, the first nine bytes correspond to the nine stickers of the U face, where byte 0 is the top left sticker (or facelet), byte 1 is the top middle sticker, byte 2 is the top right sticker, and so on. This standardization greatly eased the process of reasoning over the move application and rotation logic done within the Rust solver module, as well as the preliminary initial cube verification logic done by the Python GUI before transmission. 

Before starting any work on the software side of the project, however, I spent some time learning about the world of stepper motors and drivers. Considering my previous three projects relied on hobby servo motors, such as the MG995 and MG996R, the jump from servo motors to stepper motors was certainly a big one. Where servo motors needed at most three wires to work (Vcc, GND, and PWM), I found that getting a stepper motor to do the minimum possible work needed eighteen. I had previously worked with stepper motors in my Embedded Microcomputer Systems class during the 2025 Spring semester at Trinity University, but to a far lesser extent than this application called for. I used Aeed Musa's excellent [Rubik's Cube Robot video](https://www.youtube.com/watch?v=V8gHTKWw--Y) as the basis for my selection of stepper motor model (NEMA 17), stepper motor drivers (TMC2208), chassis STLs (can be found [here](https://www.instructables.com/Rubiks-Cube-Solver-2/)), and 24V power supply module. That said, all the wiring and software for this project is original. A significant challenge I found early on in the project was properly configuring the current limit on the TMC2208 using the on-board potentiometer. I pored over the datasheet for longer than I'd like to admit, and made some mistakes that thankfully didn't damage any of the drivers or motors I was using. Once I had a single stepper motor reliably rotating 90 degrees every second, I felt comfortable delving into the software aspect of the project.

I introduced FreeRTOS, which, despite the overarching pipeline of the project being quite linear, with each module running only once and having to wait only for the previous module to finish (Host GUI -> Rust solver -> Stepper executor), I felt it would be helpful for keeping concerns separated between modules, as well as for managing resources tracked within data structures shared between them. The principal task is the CubeProcessTask, which is in charge of receiving the UART packet from the GUI module, copying its 54 bytes from the rx_buffer into the cube_state buffer, and setting the according flags to ensure no other process is running out of turn (for example, it ensures the cube_run_motors function is executed only after the cube_run_solver function). Once I felt satisfied with the general structure of the software, I started work on the most exciting yet brain-searing section of the project: the Rust solver.

Looking back, I might have been a touch too ambitious with the scope of this project, especially considering my original goal was to be finished within a month of starting. The first milestone was setting up a Rust static library project called cube_solver. Within its lib.rs file, I specified a function that adheres to the C ABI through the use of the extern "C" directive as well as the #[no-mangle] attribute. Next, I wrote a simple shell script to run as a pre-build step on the CubeIDE, which builds the Rust crate with thumbv7em-none-eabihf as the release target to produce the static library file libcube_solver.a. The pre-build script then copies over that .a file into the Middlewares folder of the project, which is visible to the IDE. And that's basically it! All I had to do after that was to go to my project's Properties -> C/C++ Build -> Settings -> MCU/MPU GCC Linker -> Libraries to add the library search path (-L) pointing to Middlewares as well as adding -lcube_solver to the linker libraries (-l). I ran into a issue regarding a linker warning saying that the .note.GNU-stack section was missing and implied an executable stack, which the IDE treated as a build failure, even though it technically wasn't. I (mercifully) fixed this by adding the linker flags -Wl,-z,noexecstack, which explicitly marks the stack as non-executable and removes the warning, allowing the project to build successfully. After this fix, the build was clean and the Rust solver now linked properly with the STM32 project. Next, it was time to actually write the damn thing! 

I started out by ensuring the return type of the solver, the moves, will line up exactly with the ones the C module of the project is expecting, and I did this by employing the #[repr(C)] attribute on the solver_move enum. Next, I started work on laying out a robust virtual representation of the cube. This is where the standardization of the cube notation order mentioned above really came in handy. One of the parameters of the solve_cube function (the crux of the FFI I've been writing, as it exists exactly the same in memory in both the C module and Rust module of the project!) is the cube_raw pointer, which points to the beginning of the 54 initial cube bytes transmitted by the Python GUI. I hadn't written that part of the project at this point, but I knew that if I followed my notation, I could reliably treat the first nine bytes after the pointer as the nine stickers of the UP face, the next nine bytes as the DOWN face, and so on, so I didn't have to worry about that at all. The first thing my cube_solver function does is it creates a non-copying slice of the 54 cube_raw bytes, which is inherently considered unsafe, and then it copies that slice into Rust-owned stack memory; particularly, into a "stickers" array, which is the only field of the Cube struct. Upon this struct is where I wrote the main cube rotation functions, which basically just modify the associated "stickers" array. There's the rotate_face_cw function, which is only concerned with the rotation of the inner 9 stickers of a given face. Simple enough. Then there's the actual U, D, L, R, F, B functions, which all call the rotate_face function, and handles the movement of the three stickers of each of the four adjacent faces. When I started this project, I didn't even own a Rubik's cube, so reasoning over this on paper was rather difficult. To make things easier, one, I ordered a cube on Amazon, and two, I developed a comprehensive testing suite alongside the static library code. Once the main rotation functions were done and verified a week into the project, I started doing research on different solving techniques.

This is where I hit a wall. If you review the code in Rubiks-Cube-Solver/cube_solver/src/lib.rs, you'll see only the rotation functions and the C ABI function I already talked about, as well as some solver helper functions. In the end, I tried three techniques, totalling nearly 2,000 lines of code (including the testing suite), and I couldn't get any of them to work. First, I looked into the Kociemba approach, but I quickly realized that due to the self-imposed constraints (Rust no-std, limited flash space on the MCU), it wouldn't be the most feasable way to go about this. Next, I tried the CFOP approach, which seemed more promising, and it shared the same first step as the much simpler LBL approach: solve white cross. This is where the vast majority of my effort was concentrated. I won't get into the gory details (you can see some of my failed attempts within the "Failed Solvers" directory in the cube_solver Rust folder), but I basically hand-wrote a comprehensive case by case solver to move each edge piece (White-Orange, White-Green, White-Red, and White-Blue) to its corresponding final place. I later learned I was programming something called an edge-based solver, which is incredibly ineffective and demanding. Additionally, it wasn't guaranteed that the moves generated to move another edge piece wouldn't interfere with the placement of prior solved edge pieces. To help with the process, I wrote a series of helper functions; one to find the location of an edge given the its two colors, a function to identify a face given a sticker index, a function to apply a move to the virtual representation of the cube, and a function to record a move to the "out" array, which is what is being populated and passed back to the C module of the project. At this point, I had a couple of weeks left before my self imposed deadline (February 16, a month after the project began), and I still had to write the entire the Python GUI, print and assemble the robot chassis, and figure out the wiring pattern for all six stepper motor drivers. I decided to switch gears and focus on that before giving the solver another crack. 

The printing and assembly of the chassis went seamlessly, thanks to [Mr. Musa](https://www.youtube.com/@aaedmusa)'s incredibly clear and helpful documentation. The wiring design mostly went without a hitch either. I was originally going to use breadboard-esque perforated wafers to solder everything together, but due to the deadline, I went with the much more user- and time-friendly standard solderless breadboard to wire all six drivers. I used a Meanwell [LRS-25-74 24V Power Supply](https://www.digikey.com/en/products/detail/mean-well-usa-inc/LRS-75-24/7705055) to power to the drivers, and used a 5V regulator to supply voltage in series to the drivers' 5V Vio pins as well as the Nucleo's E5V internal power. Concurrently, I was working on the Python GUI aspect of the project, for which I employed the help of DeepSeek AI, to whom I entrusted parts of the Tkinter library, responsible for the visual cube layout. I wrote a simple cube validation function to be called when the "Validate Cube" button is pressed, as well when the "Send over UART" button is pressed. Similar to the Rust solver module and the C stepper executor module, the Python module uses a 54 byte array to keep track of an internal representation of the cube. I ended up using the Pyserial library to handle the sending of data to the MCU's serial port, and after a day of banging my head against a USB CDC approach for receiving data on the MCU, I found that just using STM's HAL_UART_RxCpltCallback function was significantly easier and cleaner. After much integration testing between the modules (making sure the MCU received the validated 54 bytes from the Python module, making sure the Rust module can read those 54 bytes correctly, making sure I can send a hard-coded Moves list back to the C module), I ran a comprehensive test for the entire system, and I got it to work! There's no solver, but if you modify the hard-coded moves being output from the Rust module, the corresponding stepper motor will execute those moves. 

I did return to the solver to give it one last crack a few days before the deadline, and I whipped up a version of a case-based LBL solver (as opposed to my edge-based solver) along with another whole testing suite, which inevitably also crashed and burned. Around this time, a family tragedy took place, which diverted my attention away from the project. Now, nearly a month after the deadline, I decided the best course of action should be to present the work I have done and talked about so far, call it Part 1, and move on to another project.

This is where the project stands at the moment. The solver is non-existent, so calling it a Rubik's Cube Solving Robot feels disingenuous. But everything else works! If you've read this far, I appreciate you taking an interest in this project. It certainly hasn't seen the last of me yet, and the Rust no-std Rubik's Cube Solver currently stands as the big problem looming on the horizon of my professional career. If I can figure this out, I can figure anything out, basically. Thanks for reading!

Feel free to check out my [demo video](https://drive.google.com/file/d/1MwYFFpy63CDNyF-KiC_JUUwfg-PIHqXL/view?usp=drive_link) as well as Aeed Musa's [original video](https://www.youtube.com/watch?v=V8gHTKWw--Y)
