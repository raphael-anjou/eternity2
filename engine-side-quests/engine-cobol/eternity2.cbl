      >>SOURCE FORMAT FREE
*> ===========================================================================
*>  ETERNITY II — solver and scorer, in COBOL.
*>
*>  Yes, COBOL. If Rust looked like a maintenance burden, here is the same
*>  edge-matching engine written in a language older than the puzzle, older
*>  than the web, older than nearly everyone who will ever read it. It
*>  compiles with GnuCOBOL and it actually solves.
*>
*>  It implements the core of ../engine/src/solver.rs: a path-driven (here:
*>  row-major) backtracking depth-first search with an EXPLICIT stack — COBOL
*>  has no comfortable recursion, so the search is a flat PERFORM loop over a
*>  depth cursor, exactly as the Rust solver is an explicit machine rather
*>  than a recursive function. It then scores the solved board with the same
*>  bucas convention (grey-grey interior contacts do not count).
*>
*>  Input: a piece file (argument 1) of the form
*>      <size> <colors> <seed>
*>      <u> <r> <d> <l>      (one piece per line, URDL, rotation 0)
*>  produced by ../engine/src/bin/cobol_data.rs from the Rust generator, so
*>  the COBOL output can be checked against the Rust engine piece-for-piece.
*>
*>  Conventions, kept identical to the Rust/JS contract:
*>    * Edge order URDL (up, right, down, left).
*>    * Color 0 is the grey border; interior colors 1..22.
*>    * Rotation r = clockwise quarter-turns; new[i] = old[(i+4-r) mod 4].
*>    * A board cell holds piece*4 + rot, 0-based, or "empty".
*>  COBOL tables are 1-based, so on the boundary every domain index gets +1;
*>  cells/pieces/rotations are otherwise reasoned about 0-based.
*> ===========================================================================
       IDENTIFICATION DIVISION.
       PROGRAM-ID. ETERNITY2.

       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT PIECE-FILE ASSIGN TO DYNAMIC WS-PATH
               ORGANIZATION IS LINE SEQUENTIAL
               FILE STATUS IS WS-FS.

       DATA DIVISION.
       FILE SECTION.
       FD  PIECE-FILE.
       01  PIECE-REC            PIC X(80).

       WORKING-STORAGE SECTION.
       01  WS-PATH              PIC X(256).
       01  WS-FS                PIC XX.

      *> ---- puzzle dimensions ----
       01  WS-SIZE              PIC 9(4) VALUE 0.
       01  WS-COLORS            PIC 9(4) VALUE 0.
       01  WS-SEED              PIC 9(10) VALUE 0.
       01  WS-CELLS             PIC 9(6) VALUE 0.

      *> ---- pieces in rotation 0 (URDL), 1..N ----
       01  WS-NPIECES           PIC 9(6) VALUE 0.
       01  PIECE-TABLE.
           05 PIECE-ENT OCCURS 1 TO 4096 TIMES DEPENDING ON WS-NPIECES.
              10 P-EDGE         PIC 9(4) OCCURS 4 TIMES.

      *> ---- precomputed rotation table: row = piece*4+rot (0-based) ----
      *>      stored 1-based as ROT-ENT(row+1). DISTINCT flags duplicate
      *>      rotations (e.g. a piece symmetric under a quarter turn).
       01  WS-NROWS             PIC 9(7) VALUE 0.
       01  ROT-TABLE.
           05 ROT-ENT OCCURS 1 TO 16384 TIMES DEPENDING ON WS-NROWS.
              10 R-EDGE         PIC 9(4) OCCURS 4 TIMES.
              10 R-DISTINCT     PIC 9 VALUE 1.

      *> ---- board: cell -> (row+1), or 0 when empty ----
       01  BOARD-TABLE.
           05 BOARD-CELL        PIC 9(7) OCCURS 4096 TIMES.

      *> ---- availability: piece (1..N) used? ----
       01  USED-TABLE.
           05 USED-FLAG         PIC 9 OCCURS 4096 TIMES.

      *> ---- DFS frames, one per cell in row-major visit order ----
       01  FRAME-TABLE.
           05 FRAME-ENT OCCURS 4096 TIMES.
              10 F-POS          PIC 9(6).
              10 F-CURSOR       PIC 9(7).
              10 F-PLACED       PIC 9(7).

      *> ---- search state ----
       01  WS-DEPTH             PIC S9(7) VALUE 0.
       01  WS-NODES             PIC 9(12) VALUE 0.
       01  WS-ATTEMPTS          PIC 9(15) VALUE 0.
       01  WS-BACKTRACKS        PIC 9(12) VALUE 0.
       01  WS-STATUS            PIC X(10) VALUE "RUNNING".
       01  WS-SCORE             PIC 9(6) VALUE 0.

      *> ---- scratch ----
       01  WS-I                 PIC S9(7).
       01  WS-J                 PIC S9(7).
       01  WS-R                 PIC 9.
       01  WS-PREV              PIC 9.
       01  WS-ROW               PIC 9(7).
       01  WS-OI                PIC 9(6).
       01  WS-PID               PIC 9(6).
       01  WS-POS               PIC 9(6).
       01  WS-CURSOR            PIC 9(7).
       01  WS-LIMIT             PIC 9(7).
       01  WS-PLACED-ROW        PIC S9(7).
       01  WS-FITS              PIC 9.
       01  WS-X                 PIC S9(6).
       01  WS-Y                 PIC S9(6).
       01  WS-NB                PIC S9(7).
       01  WS-EDGE-U            PIC 9(4).
       01  WS-EDGE-R            PIC 9(4).
       01  WS-EDGE-D            PIC 9(4).
       01  WS-EDGE-L            PIC 9(4).
       01  WS-TMP-EDGES.
           05 WS-TE             PIC 9(4) OCCURS 4 TIMES.
       01  WS-LINE              PIC X(80).
       01  WS-ARGC              PIC 9 VALUE 0.

      *> ---- edited (zero-suppressed) fields for human-readable output ----
       01  ED-SIZE              PIC Z(3)9.
       01  ED-COLORS            PIC Z(3)9.
       01  ED-SEED              PIC Z(9)9.
       01  ED-PLACED            PIC Z(6)9.
       01  ED-SCORE             PIC Z(5)9.
       01  ED-NODES             PIC Z(11)9.
       01  ED-ATTEMPTS          PIC Z(14)9.
       01  ED-BACKTRACKS        PIC Z(11)9.

       PROCEDURE DIVISION.
       MAIN.
           ACCEPT WS-PATH FROM COMMAND-LINE
           IF WS-PATH = SPACES
               DISPLAY "usage: eternity2 <piece-file>"
               MOVE 2 TO RETURN-CODE
               STOP RUN
           END-IF
           PERFORM LOAD-PUZZLE
           PERFORM BUILD-ROTATIONS
           PERFORM INIT-SEARCH
           PERFORM RUN-SEARCH
           PERFORM SCORE-BOARD
           PERFORM WRITE-REPORT
           STOP RUN.

      *> -----------------------------------------------------------------
      *> Read the piece file. Line 1: "<size> <colors> <seed>". Remaining
      *> lines: one piece each, "<u> <r> <d> <l>".
      *> -----------------------------------------------------------------
       LOAD-PUZZLE.
           OPEN INPUT PIECE-FILE
           IF WS-FS NOT = "00"
               DISPLAY "cannot open " FUNCTION TRIM(WS-PATH)
               MOVE 1 TO RETURN-CODE
               STOP RUN
           END-IF
           READ PIECE-FILE INTO WS-LINE
               AT END CONTINUE
           END-READ
           UNSTRING WS-LINE DELIMITED BY ALL SPACES
               INTO WS-SIZE WS-COLORS WS-SEED
           COMPUTE WS-CELLS = WS-SIZE * WS-SIZE
           MOVE 0 TO WS-NPIECES
           PERFORM UNTIL WS-FS NOT = "00"
               READ PIECE-FILE INTO WS-LINE
                   AT END EXIT PERFORM
               END-READ
               IF WS-LINE NOT = SPACES
                   ADD 1 TO WS-NPIECES
                   UNSTRING WS-LINE DELIMITED BY ALL SPACES
                       INTO P-EDGE(WS-NPIECES, 1)
                            P-EDGE(WS-NPIECES, 2)
                            P-EDGE(WS-NPIECES, 3)
                            P-EDGE(WS-NPIECES, 4)
               END-IF
           END-PERFORM
           CLOSE PIECE-FILE
           IF WS-NPIECES NOT = WS-CELLS
               DISPLAY "piece/cell count mismatch: "
                   WS-NPIECES " vs " WS-CELLS
               MOVE 1 TO RETURN-CODE
               STOP RUN
           END-IF.

      *> -----------------------------------------------------------------
      *> Precompute the 4 rotations of every piece into ROT-ENT, and mark
      *> rotations that duplicate a lower one (so the search skips them).
      *> row (0-based) = (pid-1)*4 + r ; stored at ROT-ENT(row+1).
      *> -----------------------------------------------------------------
       BUILD-ROTATIONS.
           COMPUTE WS-NROWS = WS-NPIECES * 4
           PERFORM VARYING WS-I FROM 1 BY 1 UNTIL WS-I > WS-NPIECES
               PERFORM VARYING WS-R FROM 0 BY 1 UNTIL WS-R > 3
                   MOVE P-EDGE(WS-I, 1) TO WS-TE(1)
                   MOVE P-EDGE(WS-I, 2) TO WS-TE(2)
                   MOVE P-EDGE(WS-I, 3) TO WS-TE(3)
                   MOVE P-EDGE(WS-I, 4) TO WS-TE(4)
                   PERFORM ROTATE-TMP
                   COMPUTE WS-ROW = (WS-I - 1) * 4 + WS-R + 1
                   MOVE WS-TE(1) TO R-EDGE(WS-ROW, 1)
                   MOVE WS-TE(2) TO R-EDGE(WS-ROW, 2)
                   MOVE WS-TE(3) TO R-EDGE(WS-ROW, 3)
                   MOVE WS-TE(4) TO R-EDGE(WS-ROW, 4)
                   MOVE 1 TO R-DISTINCT(WS-ROW)
                   PERFORM VARYING WS-PREV FROM 0 BY 1
                           UNTIL WS-PREV >= WS-R
                       COMPUTE WS-J = (WS-I - 1) * 4 + WS-PREV + 1
                       IF R-EDGE(WS-J,1) = R-EDGE(WS-ROW,1)
                          AND R-EDGE(WS-J,2) = R-EDGE(WS-ROW,2)
                          AND R-EDGE(WS-J,3) = R-EDGE(WS-ROW,3)
                          AND R-EDGE(WS-J,4) = R-EDGE(WS-ROW,4)
                           MOVE 0 TO R-DISTINCT(WS-ROW)
                           EXIT PERFORM
                       END-IF
                   END-PERFORM
               END-PERFORM
           END-PERFORM.

      *> Rotate WS-TE in place by WS-R clockwise quarter-turns.
      *> new[i] = old[(i+4-r) mod 4]; written per-r so it is obviously total.
       ROTATE-TMP.
           MOVE WS-TE(1) TO WS-EDGE-U
           MOVE WS-TE(2) TO WS-EDGE-R
           MOVE WS-TE(3) TO WS-EDGE-D
           MOVE WS-TE(4) TO WS-EDGE-L
           EVALUATE WS-R
               WHEN 1
                   MOVE WS-EDGE-L TO WS-TE(1)
                   MOVE WS-EDGE-U TO WS-TE(2)
                   MOVE WS-EDGE-R TO WS-TE(3)
                   MOVE WS-EDGE-D TO WS-TE(4)
               WHEN 2
                   MOVE WS-EDGE-D TO WS-TE(1)
                   MOVE WS-EDGE-L TO WS-TE(2)
                   MOVE WS-EDGE-U TO WS-TE(3)
                   MOVE WS-EDGE-R TO WS-TE(4)
               WHEN 3
                   MOVE WS-EDGE-R TO WS-TE(1)
                   MOVE WS-EDGE-D TO WS-TE(2)
                   MOVE WS-EDGE-L TO WS-TE(3)
                   MOVE WS-EDGE-U TO WS-TE(4)
               WHEN OTHER
                   CONTINUE
           END-EVALUATE.

      *> -----------------------------------------------------------------
      *> Empty board, no hints (generated puzzles carry none); one frame per
      *> cell in row-major order. piece_order is the identity (1..N).
      *> -----------------------------------------------------------------
       INIT-SEARCH.
           PERFORM VARYING WS-I FROM 1 BY 1 UNTIL WS-I > WS-CELLS
               MOVE 0 TO BOARD-CELL(WS-I)
               MOVE 0 TO USED-FLAG(WS-I)
               COMPUTE F-POS(WS-I) = WS-I - 1
               MOVE 0 TO F-CURSOR(WS-I)
               MOVE 0 TO F-PLACED(WS-I)
           END-PERFORM
           MOVE 0 TO WS-DEPTH
           MOVE 0 TO WS-NODES
           MOVE 0 TO WS-ATTEMPTS
           MOVE 0 TO WS-BACKTRACKS
           MOVE "RUNNING" TO WS-STATUS.

      *> -----------------------------------------------------------------
      *> The explicit-stack DFS. Mirrors solver.rs step(): one iteration is
      *> one placement or one backtrack. Runs to completion (SOLVED or
      *> EXHAUSTED) rather than on a budget, since this is a CLI batch run.
      *> -----------------------------------------------------------------
       RUN-SEARCH.
           PERFORM UNTIL WS-STATUS NOT = "RUNNING"
               IF WS-DEPTH = WS-CELLS
                   MOVE "SOLVED" TO WS-STATUS
                   EXIT PERFORM
               END-IF
               COMPUTE WS-I = WS-DEPTH + 1
               MOVE F-POS(WS-I) TO WS-POS
               MOVE F-CURSOR(WS-I) TO WS-CURSOR
               MOVE WS-NROWS TO WS-LIMIT
               MOVE -1 TO WS-PLACED-ROW

      *>         scan candidate (piece, rotation) rows from the cursor
               PERFORM UNTIL WS-CURSOR >= WS-LIMIT
                   COMPUTE WS-OI = WS-CURSOR / 4
                   COMPUTE WS-R = FUNCTION MOD(WS-CURSOR, 4)
                   ADD 1 TO WS-CURSOR
                   COMPUTE WS-PID = WS-OI + 1
                   IF USED-FLAG(WS-PID) = 1
      *>                 skip this piece's remaining rotations
                       COMPUTE WS-CURSOR =
                           FUNCTION INTEGER((WS-CURSOR + 3) / 4) * 4
                   ELSE
                       COMPUTE WS-ROW = WS-OI * 4 + WS-R + 1
                       IF R-DISTINCT(WS-ROW) = 1
                           ADD 1 TO WS-ATTEMPTS
                           PERFORM CHECK-FIT
                           IF WS-FITS = 1
                               MOVE WS-ROW TO WS-PLACED-ROW
                               EXIT PERFORM
                           END-IF
                       END-IF
                   END-IF
               END-PERFORM

               IF WS-PLACED-ROW >= 0
                   COMPUTE WS-PID = ((WS-PLACED-ROW - 1) / 4) + 1
                   COMPUTE WS-J = WS-POS + 1
                   MOVE WS-PLACED-ROW TO BOARD-CELL(WS-J)
                   MOVE 1 TO USED-FLAG(WS-PID)
                   MOVE WS-CURSOR TO F-CURSOR(WS-I)
                   MOVE WS-PLACED-ROW TO F-PLACED(WS-I)
                   ADD 1 TO WS-DEPTH
                   ADD 1 TO WS-NODES
               ELSE
                   MOVE 0 TO F-CURSOR(WS-I)
                   IF WS-DEPTH = 0
                       MOVE "EXHAUSTED" TO WS-STATUS
                       EXIT PERFORM
                   END-IF
                   SUBTRACT 1 FROM WS-DEPTH
                   COMPUTE WS-J = WS-DEPTH + 1
                   MOVE F-PLACED(WS-J) TO WS-ROW
                   MOVE 0 TO F-PLACED(WS-J)
                   COMPUTE WS-PID = ((WS-ROW - 1) / 4) + 1
                   COMPUTE WS-I = F-POS(WS-J) + 1
                   MOVE 0 TO BOARD-CELL(WS-I)
                   MOVE 0 TO USED-FLAG(WS-PID)
                   ADD 1 TO WS-BACKTRACKS
               END-IF
           END-PERFORM.

      *> -----------------------------------------------------------------
      *> Does ROT-ENT(WS-ROW) fit at WS-POS? rim/interior border rules plus
      *> matching every already-placed neighbor. Sets WS-FITS to 0/1.
      *> -----------------------------------------------------------------
       CHECK-FIT.
           MOVE R-EDGE(WS-ROW, 1) TO WS-EDGE-U
           MOVE R-EDGE(WS-ROW, 2) TO WS-EDGE-R
           MOVE R-EDGE(WS-ROW, 3) TO WS-EDGE-D
           MOVE R-EDGE(WS-ROW, 4) TO WS-EDGE-L
           COMPUTE WS-X = FUNCTION MOD(WS-POS, WS-SIZE)
           COMPUTE WS-Y = WS-POS / WS-SIZE
           MOVE 1 TO WS-FITS

      *>     rim cells must have a grey (0) outward edge; interior must not
           IF (WS-Y = 0 AND WS-EDGE-U NOT = 0)
              OR (WS-Y NOT = 0 AND WS-EDGE-U = 0)
               MOVE 0 TO WS-FITS
           END-IF
           IF WS-FITS = 1 AND
              ((WS-Y = WS-SIZE - 1 AND WS-EDGE-D NOT = 0)
               OR (WS-Y NOT = WS-SIZE - 1 AND WS-EDGE-D = 0))
               MOVE 0 TO WS-FITS
           END-IF
           IF WS-FITS = 1 AND
              ((WS-X = 0 AND WS-EDGE-L NOT = 0)
               OR (WS-X NOT = 0 AND WS-EDGE-L = 0))
               MOVE 0 TO WS-FITS
           END-IF
           IF WS-FITS = 1 AND
              ((WS-X = WS-SIZE - 1 AND WS-EDGE-R NOT = 0)
               OR (WS-X NOT = WS-SIZE - 1 AND WS-EDGE-R = 0))
               MOVE 0 TO WS-FITS
           END-IF

      *>     neighbor above: its DOWN (edge 3) must equal our UP
           IF WS-FITS = 1 AND WS-Y > 0
               COMPUTE WS-NB = WS-POS - WS-SIZE + 1
               IF BOARD-CELL(WS-NB) > 0
                   IF R-EDGE(BOARD-CELL(WS-NB), 3) NOT = WS-EDGE-U
                       MOVE 0 TO WS-FITS
                   END-IF
               END-IF
           END-IF
      *>     neighbor below: its UP (edge 1) must equal our DOWN
           IF WS-FITS = 1 AND WS-Y < WS-SIZE - 1
               COMPUTE WS-NB = WS-POS + WS-SIZE + 1
               IF BOARD-CELL(WS-NB) > 0
                   IF R-EDGE(BOARD-CELL(WS-NB), 1) NOT = WS-EDGE-D
                       MOVE 0 TO WS-FITS
                   END-IF
               END-IF
           END-IF
      *>     neighbor left: its RIGHT (edge 2) must equal our LEFT
           IF WS-FITS = 1 AND WS-X > 0
               COMPUTE WS-NB = WS-POS - 1 + 1
               IF BOARD-CELL(WS-NB) > 0
                   IF R-EDGE(BOARD-CELL(WS-NB), 2) NOT = WS-EDGE-L
                       MOVE 0 TO WS-FITS
                   END-IF
               END-IF
           END-IF
      *>     neighbor right: its LEFT (edge 4) must equal our RIGHT
           IF WS-FITS = 1 AND WS-X < WS-SIZE - 1
               COMPUTE WS-NB = WS-POS + 1 + 1
               IF BOARD-CELL(WS-NB) > 0
                   IF R-EDGE(BOARD-CELL(WS-NB), 4) NOT = WS-EDGE-R
                       MOVE 0 TO WS-FITS
                   END-IF
               END-IF
           END-IF.

      *> -----------------------------------------------------------------
      *> Count matched interior edges (right + down neighbors), skipping
      *> grey-grey contacts. Same convention as score_board in solver.rs.
      *> -----------------------------------------------------------------
       SCORE-BOARD.
           MOVE 0 TO WS-SCORE
           PERFORM VARYING WS-Y FROM 0 BY 1 UNTIL WS-Y >= WS-SIZE
               PERFORM VARYING WS-X FROM 0 BY 1 UNTIL WS-X >= WS-SIZE
                   COMPUTE WS-I = WS-Y * WS-SIZE + WS-X + 1
                   IF BOARD-CELL(WS-I) > 0
                       IF WS-X + 1 < WS-SIZE
                           COMPUTE WS-J = WS-I + 1
                           IF BOARD-CELL(WS-J) > 0
                               IF R-EDGE(BOARD-CELL(WS-I), 2) =
                                  R-EDGE(BOARD-CELL(WS-J), 4)
                                  AND R-EDGE(BOARD-CELL(WS-I), 2) NOT = 0
                                   ADD 1 TO WS-SCORE
                               END-IF
                           END-IF
                       END-IF
                       IF WS-Y + 1 < WS-SIZE
                           COMPUTE WS-J = WS-I + WS-SIZE
                           IF BOARD-CELL(WS-J) > 0
                               IF R-EDGE(BOARD-CELL(WS-I), 3) =
                                  R-EDGE(BOARD-CELL(WS-J), 1)
                                  AND R-EDGE(BOARD-CELL(WS-I), 3) NOT = 0
                                   ADD 1 TO WS-SCORE
                               END-IF
                           END-IF
                       END-IF
                   END-IF
               END-PERFORM
           END-PERFORM.

      *> -----------------------------------------------------------------
      *> One machine-parseable line, matching the Rust "RESULT" reference.
      *> -----------------------------------------------------------------
       WRITE-REPORT.
           MOVE WS-SIZE       TO ED-SIZE
           MOVE WS-COLORS     TO ED-COLORS
           MOVE WS-SEED       TO ED-SEED
           MOVE WS-DEPTH      TO ED-PLACED
           MOVE WS-SCORE      TO ED-SCORE
           MOVE WS-NODES      TO ED-NODES
           MOVE WS-ATTEMPTS   TO ED-ATTEMPTS
           MOVE WS-BACKTRACKS TO ED-BACKTRACKS
           DISPLAY "RESULT " FUNCTION TRIM(ED-SIZE)
               " " FUNCTION TRIM(ED-COLORS)
               " " FUNCTION TRIM(ED-SEED)
               ": status=" FUNCTION TRIM(WS-STATUS)
               " placed=" FUNCTION TRIM(ED-PLACED)
               " score=" FUNCTION TRIM(ED-SCORE)
               " nodes=" FUNCTION TRIM(ED-NODES)
               " attempts=" FUNCTION TRIM(ED-ATTEMPTS)
               " backtracks=" FUNCTION TRIM(ED-BACKTRACKS).
