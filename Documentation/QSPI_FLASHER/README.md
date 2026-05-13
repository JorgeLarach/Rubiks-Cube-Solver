## May 12, 2026

This folder contains the main.c and other associated files for a separate STM32 project concerned with flashing ~4.5MB of lookup tables used in the Rust solver's heuristic and search algorithms onto a W25Q128JVSSIQ NOR Flash breakout board. This is a single-use program, much like the table generation code in cube_solver.
Tables info and their purpose

QSPI External Flash (W25Q128 = 16MB):
| Table                | Size (B)             | Purpose 
| -----                | ------------         | -------  
| FLIP_UDSLICE_TABLE   |      1,013,760 bytes | Phase 1 heuristic
| CORNERS_SLICE2_TABLE |        967,680 bytes | Phase 2 heuristic
| EDGES_SLICE2_TABLE   |        967,680 bytes | Phase 2 heuristic
| CP_MOVE_TABLE        |        806,400 bytes | Phase 2 search
| EP_MOVE_TABLE        |        806,400 bytes | Phase 2 search

Total QSPI: 4,561,920 bytes

This code is not executable here, it is only included for documentation purposes. If you are interested in more information about this "helper" project, reach out at jorgelarachesp@gmail.com 