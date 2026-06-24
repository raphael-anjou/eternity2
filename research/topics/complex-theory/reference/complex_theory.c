/* Eternity 2 Puzzle

   An implementation of Brendan Owen's Complex Theory
   by Peter McGavin, 15 Jan 2024

   References:
     https://groups.io/g/eternity2/message/5197
     https://groups.io/g/eternity2/message/5209
     https://groups.io/g/eternity2/files/Peter%20McGavin/complex_theory.pdf

   Uses libgmp, the GNU Multiple Precision Arithmetic library

   Compile with:
     gcc -o complex_theory complex_theory.c -lgmp -lm -O3 -Wall -s
*/

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <math.h>
#include <ctype.h>
#include <gmp.h>

/**********************************************************************/
/* define the puzzle, piece placement order, and hint locations */

/* puzzle size - number of rows and columns */
#define NROWS 16
#define NCOLS 16

/* number of border edge types and middle edge types */
#define NBT    5
#define NMT   17

/* number of border joins of each type */
/*   == half the number of border edges of each type*/
static const int bt[NBT] =
  {12, 12, 12, 12, 12};

/* number of middle joins of each type */
/*   == half the number of middle edges of each type */
static const int mt[NMT] =
  {24, 25, 25, 24, 25, 25, 24, 25, 25, 24, 25, 25, 24, 25, 25, 25, 25};

/* the order/path of placing pieces on the board */
/* coordinates are as defined on the official eternity 2 board */
/* for example, P16 is the bottom right corner */
/* this example is scan-row starting from P1 (bottom left) */
const char *cpath = {
  "P1,P2,P3,P4,P5,P6,P7,P8,P9,P10,P11,P12,P13,P14,P15,P16,"
  "O1,O2,O3,O4,O5,O6,O7,O8,O9,O10,O11,O12,O13,O14,O15,O16,"
  "N1,N2,N3,N4,N5,N6,N7,N8,N9,N10,N11,N12,N13,N14,N15,N16,"
  "M1,M2,M3,M4,M5,M6,M7,M8,M9,M10,M11,M12,M13,M14,M15,M16,"
  "L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,L15,L16,"
  "K1,K2,K3,K4,K5,K6,K7,K8,K9,K10,K11,K12,K13,K14,K15,K16,"
  "J1,J2,J3,J4,J5,J6,J7,J8,J9,J10,J11,J12,J13,J14,J15,J16,"
  "I1,I2,I3,I4,I5,I6,I7,I8,I9,I10,I11,I12,I13,I14,I15,I16,"
  "H1,H2,H3,H4,H5,H6,H7,H8,H9,H10,H11,H12,H13,H14,H15,H16,"
  "G1,G2,G3,G4,G5,G6,G7,G8,G9,G10,G11,G12,G13,G14,G15,G16,"
  "F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,F13,F14,F15,F16,"
  "E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,"
  "D1,D2,D3,D4,D5,D6,D7,D8,D9,D10,D11,D12,D13,D14,D15,D16,"
  "C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,C15,C16,"
  "B1,B2,B3,B4,B5,B6,B7,B8,B9,B10,B11,B12,B13,B14,B15,B16,"
  "A1,A2,A3,A4,A5,A6,A7,A8,A9,A10,A11,A12,A13,A14,A15,A16"
};

/* hints (or clues) */
#define NMHINTS 1     /* number of hints on middle squares */
#define NBHINTS 0     /* number of hints on border squares */
#define NCHINTS 0     /* number of hints on corner squares */
const char *mh = {"I8,N3,C14,N14,C3"}; /* middle hint squares*/
const char *bh = NULL;   /* border hint squares */
const char *ch = NULL;   /* corner hint squares */

/**********************************************************************/
/* pre-calculated constants */

#define N (NROWS * NCOLS)
#define NC 4                          /* number of corner squares/tiles */
#define NB (2*(NROWS-2)+2*(NCOLS-2))  /* number of border squares/tiles */
#define NM ((NROWS-2)*(NCOLS-2))      /* number of middle squares/tiles */
#define NMIDDLE ((NROWS-1)*(NCOLS-2)+(NROWS-2)*(NCOLS-1)) /* n middle joins */
#define NBORDER (2*(NCOLS-1)+2*(NROWS-1))         /* number of border joins */

/* limits */
#define MAXMHINTS 16
#define MAXBHINTS 16
#define MAXCHINTS 4

#define FP_PRECISION_BITS 64

/**********************************************************************/
/* types */

enum loc {
  MIDDLE,
  BORDER,
  CORNER,
  OUTSIDE,
};

struct squareinfo {
  enum loc loc;
  int piece;
};

struct hint {
  int piece, row, col;
};

struct coord {
  int row;
  int col;
};

/**********************************************************************/
static __attribute__ ((noreturn)) void fatal (char *msg, ...)
{
  va_list argptr;

  va_start (argptr, msg);
  vfprintf (stderr, msg, argptr);
  fprintf (stderr, "\n");
  va_end (argptr);
  exit (EXIT_FAILURE);
}

/**********************************************************************/
static void myassert (int expr, char *msg, ...)
{
  va_list argptr;

  if (!expr) {
    va_start (argptr, msg);
    vfprintf (stderr, msg, argptr);
    fprintf (stderr, "\n");
    va_end (argptr);
    exit (EXIT_FAILURE);
  }
}

/**********************************************************************/
static void parse_coords (const char *p, struct coord *coords, int size)
{
  int i, n;
  char a;

  if (p == NULL)
    return;
  i = 0;
  for (;;) {
    a = toupper(*p++);
    myassert (a >= 'A' && a <= 'A' + NROWS - 1, "'A'-'%c' expected, found %c",
	      'A' + NROWS - 1, a);
    coords[i].row = a - 'A';
    myassert (isdigit(*p), "Digit expected, found %c", *p);
    n = *p++ - '0';
    while (isdigit(*p))
      n = 10 * n + *p++ - '0';
    myassert (n > 0 && n <= NCOLS, "1-%d expected, found %d", NCOLS, n);
    coords[i].col = n - 1;
    if (++i == size)
      break;
    myassert (*p == ',', "comma expected, found %c", *p);
    p++;
  }
}

/**********************************************************************/
static void factorial (mpf_t v, int n)
/* return v = n! */
{
  static mpf_t fact[621];
  static int first = 1;

  myassert (n >= 0 && n < 621, "factorial(%d)", n);
  if (first) {
    int i;
    mpf_init_set_ui (fact[0], 1);
    for (i = 1; i < 621; i++) {
      mpf_init (fact[i]);
      mpf_mul_ui (fact[i], fact[i-1], i);
    }
    first = 0;
  }
  mpf_set (v, fact[n]);
}

/**********************************************************************/
static void perm (mpf_t v, int n, int k)
/* return v = P(n,k) = n!/(n-k)! */
{
  int i;

  myassert (n >= k && k >= 0, "perm(%d,%d)!", n, k);
  mpf_set_ui (v, 1.0);
  for (i = n - k + 1; i <= n; i++)
    mpf_mul_ui (v, v, i);
}

/**********************************************************************/
static void comb (mpf_t v, int n, int k)
/* return v = C(n,k) = n!/((n-k!)*k!) */
{
  mpf_t den;

  myassert (n >= k && k >= 0, "comb(%d,%d)!", n, k);
  perm (v, n, k);
  mpf_init (den);
  factorial (den, k);
  mpf_div (v, v, den);
  mpf_clear (den);
}

/**********************************************************************/
static void validm (mpf_t v, int i, int m, int *n)
/* return v = number of valid combinations of m middle joins can be made
   using 2m edges of types 0..i-1.  There are 2n[i-1] edges of type i.  */
{
  int j;
  static mpf_t res[18][421];
  static int first = 1;

  myassert (i >= 0 && i < 18 && m >= 0 && m < 421, "validm(%d,%d)!", i, m);
  if (first) {
    int i, m;
    for (i = 0; i < 18; i++)
      for (m = 0; m < 421; m++)
	mpf_init_set_si (res[i][m], -1);
    first = 0;
  }
  if (mpf_cmp_si (res[i][m], -1) != 0) {
    mpf_set (v, res[i][m]);
    return;
  }
  if (m == 0)
    mpf_set_si (v, 1);
  else {
    mpf_set_si (v, 0);
    if (i > 0 && m > 0) {
      mpf_t t1, t2;
      mpf_init (t1);
      mpf_init (t2);
      for (j = 0; j <= n[i-1] && j <= m; j++) {
	validm (t1, i - 1, m - j, n);
	perm (t2, 2 * n[i-1], 2 * j);
	mpf_mul (t1, t1, t2);
	comb (t2, m, j);
	mpf_mul (t1, t1, t2);
	mpf_add (v, v, t1);
      }
      mpf_clear (t1);
      mpf_clear (t2);
    }
  }
  mpf_set (res[i][m], v);
}

/**********************************************************************/
static void validb (mpf_t v, int i, int b, int *n)
/* return v = number of valid combinations of b border joins can be made
   using 2b edges of types 0..i-1.  There are n[i-1] left edges of type i
   and n[i-1] right edges of type i.  */
{
  int j;
  static mpf_t res[6][61];
  static int first = 1;

  myassert (i >= 0 && i < 6 && b >= 0 && b < 61, "validb(%d,%d)!", i, b);
  if (first) {
    int i, b;
    for (i = 0; i < 6; i++)
      for (b = 0; b < 61; b++)
	mpf_init_set_si (res[i][b], -1);
    first = 0;
  }
  if (mpf_cmp_si (res[i][b], -1) != 0) {
    mpf_set (v, res[i][b]);
    return;
  }
  if (b == 0)
    mpf_set_si (v, 1);
  else {
    mpf_set_si (v, 0);
    if (i > 0 && b > 0) {
      mpf_t t1, t2;
      mpf_init (t1);
      mpf_init (t2);
      for (j = 0; j <= n[i-1] && j <= b; j++) {
	validb (t1, i - 1, b - j, n);
	perm (t2, n[i-1], j);
	mpf_mul (t2, t2, t2);
	mpf_mul (t1, t1, t2);
	comb (t2, b, j);
	mpf_mul (t1, t1, t2);
	mpf_add (v, v, t1);
      }
      mpf_clear (t1);
      mpf_clear (t2);
    }
  }
  mpf_set (res[i][b], v);
}

/**********************************************************************/
static void stat (struct squareinfo square[NROWS][NCOLS], mpf_t s2)
{
  int row, col, m, b, nc, nb, nm, nbt, nmt, i;
  mpf_t p, pb2, pm2, t1, t2, t3, t4;

  nc = 0; /* number of occupied corner squares */
  nb = 0; /* number of occupied border squares */
  nm = 0; /* number of occupied middle squares */
  for (row = 0; row < NROWS; row++) {
    for (col = 0; col < NCOLS; col++) {
      struct squareinfo *sq = &square[row][col];
      if (sq->piece >= 0) {
	if (sq->loc == CORNER)
	  nc++;
	else if (sq->loc == BORDER)
	  nb++;
	else if (sq->loc == MIDDLE)
	  nm++;
	else
	  fatal ("Ooops in stat()");
      }
    }
  }
  m = 0; /* number of completed middle joins */
  b = 0; /* number of completed border joins */
  for (row = 0; row < NROWS; row++) {
    for (col = 1; col < NCOLS; col++) {
      struct squareinfo *bl = &square[row][col-1];
      struct squareinfo *br = &square[row][col];
      if (bl->piece >= 0 && br->piece >= 0) {
	if (row == 0 || row == NROWS - 1)
	  b++;
	else
	  m++;
      }
    }
  }
  for (row = 1; row < NROWS; row++) {
    for (col = 0; col < NCOLS; col++) {
      struct squareinfo *bu = &square[row-1][col];
      struct squareinfo *bd = &square[row][col];
      if (bu->piece >= 0 && bd->piece >= 0) {
	if (col == 0 || col == NCOLS - 1)
	  b++;
	else
	  m++;
      }
    }
  }
  nmt = 0; /* 2*nmt = number of middle edges on tiles */
  for (i = 0; i < NMT; i++)
    nmt += mt[i];
  myassert (nmt >= NMIDDLE, "nmt < NMIDDLE, %d, %d", nmt, NMIDDLE);
  nbt = 0; /* 2*nbt = number of border edges on tiles */
  for (i = 0; i < NBT; i++)
    nbt += bt[i];
  myassert (nbt >= NBORDER, "nbt < NBORDER, %d, %d", nbt, NBORDER);
  if (nmt > NMIDDLE || nbt > NBORDER)
    printf ("Warning: More pairs of tile edges than board joins !!!\n");

  mpf_init (p);
  mpf_init (pb2);
  mpf_init (pm2);
  mpf_init (t1);
  mpf_init (t2);
  mpf_init (t3);
  mpf_init (t4);

  /* p = permutations of placing pieces at this depth, P(c,e,i) */
  perm (t1, NC - NCHINTS, nc - NCHINTS);
  perm (t2, NB - NBHINTS, nb - NBHINTS);
  perm (t3, NM - NMHINTS, nm - NMHINTS);
  mpf_set_d (t4, pow (4.0, nm - NMHINTS));
  mpf_mul (p, t1, t2);
  mpf_mul (p, p, t3);
  mpf_mul (p, p, t4);

  /* pm2 = complex prob that m middle joins are valid with NMT middle types */
  validm (t1, NMT, m, (int *)mt);
  perm (t2, 2 * nmt, 2 * m);
  mpf_div (pm2, t1, t2);

  /* pb2 = complex prob that b border joins are valid with NBT border types */
  validb (t1, NBT, b, (int *)bt);
  perm (t2, nbt, b);
  mpf_mul (t3, t2, t2);
  mpf_div (pb2, t1, t3);

  /* s2 = complex estimated solutions at this depth */
  mpf_mul (s2, pm2, pb2);
  mpf_mul (s2, p, s2);

  mpf_clear (p);
  mpf_clear (pb2);
  mpf_clear (pm2);
  mpf_clear (t1);
  mpf_clear (t2);
  mpf_clear (t3);
  mpf_clear (t4);
}

/**********************************************************************/
int main (int argc, char *argv[])
{
  int row, col, i, p;
  struct squareinfo square[NROWS][NCOLS], *sq;
  struct coord path[N];
  struct coord mhint[MAXMHINTS], bhint[MAXBHINTS], chint[MAXCHINTS];
  mpf_t s2[N+1], sum_s2;

  mpf_set_default_prec (FP_PRECISION_BITS);

  printf ("------------------------------------------------------------------------\n");
  parse_coords (cpath, path, N);
  parse_coords (mh, mhint, NMHINTS);
  parse_coords (bh, bhint, NBHINTS);
  parse_coords (ch, chint, NCHINTS);

  for (row = 0; row < NROWS; row++) {
    for (col = 0; col < NCOLS; col++) {
      sq = &square[row][col];
      sq->piece = -1;
      if (row == 0 || row == NROWS - 1 || col == 0 || col == NCOLS - 1)
	if ((row == 0 || row == NROWS - 1) && (col == 0 || col == NCOLS - 1))
	  sq->loc = CORNER;
	else
	  sq->loc = BORDER;
      else
	sq->loc = MIDDLE;
    }
  }

  printf ("  p  sq   num-solns  cum-solns\n");
  mpf_init_set_si (sum_s2, 0);
  for (i = 0; i < N + 1; i++)
    mpf_init_set_si (s2[i], 1);
  for (p = 0, i = 0; p < N; i++) {
    int x, y;
    if (i < NMHINTS) {
      x = mhint[i].row;
      y = mhint[i].col;
    } else if (i < NMHINTS + NBHINTS) {
      x = bhint[i - NMHINTS].row;
      y = bhint[i - NMHINTS].col;
    } else if (i < NMHINTS + NBHINTS + NCHINTS) {
      x = chint[i - NMHINTS - NBHINTS].row;
      y = chint[i - NMHINTS - NBHINTS].col;
    } else {
      x = path[i - NMHINTS - NBHINTS - NCHINTS].row;
      y = path[i - NMHINTS - NBHINTS - NCHINTS].col;
    }
    sq = &square[x][y];
    if (sq->piece < 0) {
      sq->piece = p++;
      if (i < NMHINTS + NBHINTS + NCHINTS)
	mpf_set_si (s2[p], 1);
      else
	stat (square, s2[p]);
      mpf_add (sum_s2, sum_s2, s2[p]);
      printf ("%3d %c%2.2d  %10.5g %10.5g\n",
	      p, 'A' + x, y + 1,
	      mpf_get_d(s2[p]), mpf_get_d(sum_s2));
    }
  }

  printf ("------------------------------------------------------------------------\n");

  mpf_clear (sum_s2);
  for (i = 0; i < N + 1; i++)
    mpf_clear (s2[i]);

  return EXIT_SUCCESS;
}

/**********************************************************************/
