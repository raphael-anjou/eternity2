/* gap 4, toy check. Exact search trees on GENERATED framed edge-matching boards (toys, not Eternity II).
 * One propagation core (bitset domains over (tile, rotation) values), several branching rules.
 * usage: toy_orders n cf ci seed cap
 * Output: one line per variant: name, branches tried (tree size), solutions, censored flag, seconds.
 * Tree size = number of branches tried (a value put in a cell, a placement tried for a tile, a colour tried on a joint).
 * Propagation levels: PLAIN (none: a cell is only checked against decided neighbours and used tiles),
 *   FC (forward checking: fail when any cell has no value left), AC (arc consistency on colours to a fixpoint,
 *   plus "a cell with one value left removes that tile elsewhere").
 * Branching rules: ROW (row scan), DOM (smallest domain anywhere), YK (smallest domain among staircase corners
 *   within K open rows: Y1 is row scan), MIX (exact-cover rule: smallest of cell domains and tile placement counts),
 *   TILE (always the tile with fewest placements), EDGE (joint with fewest colours, AC only). */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#define MAXC 81
#define W 6
typedef struct { uint64_t w[W]; } bs;
typedef struct { bs dom[MAXC]; unsigned char dec[MAXC], fix[MAXC]; int ndec; } St;
static int n, NC, NV, NCOL;
static int col[4*MAXC][4];
static bs bycol[4][64], tilemask[MAXC], ALL0;
static int nb[MAXC][4]; /* neighbour of cell in direction d (0=N,1=E,2=S,3=W), -1 if wall */
static uint64_t tried, cap, sols; static int censored;
static uint64_t prof[4*MAXC+8];
enum { PLAIN, FC, AC };
enum { ROW, DOM, YK, MIX, TILE, EDGE };
static int PROP, RULE, KOPEN;
static inline int bs_empty(const bs *a){ uint64_t x=0; for(int i=0;i<W;i++) x|=a->w[i]; return x==0; }
static inline int bs_cnt(const bs *a){ int c=0; for(int i=0;i<W;i++) c+=__builtin_popcountll(a->w[i]); return c; }
static inline int bs_and_nz(const bs *a,const bs *b){ uint64_t x=0; for(int i=0;i<W;i++) x|=a->w[i]&b->w[i]; return x!=0; }
static inline int bs_first(const bs *a){ for(int i=0;i<W;i++) if(a->w[i]) return i*64+__builtin_ctzll(a->w[i]); return -1; }
static uint64_t rs;
static uint64_t rnd(void){ rs^=rs<<13; rs^=rs>>7; rs^=rs<<17; return rs; }

/* revise x against its neighbour y lying in direction d of x. returns 0 unchanged, 1 changed, -1 emptied */
static int revise(St *s,int x,int d){
  int y=nb[x][d]; int od=(d+2)&3; bs allow; memset(&allow,0,sizeof allow);
  for(int k=1;k<=NCOL;k++) if(bs_and_nz(&s->dom[y],&bycol[od][k])) for(int i=0;i<W;i++) allow.w[i]|=bycol[d][k].w[i];
  int ch=0; uint64_t any=0;
  for(int i=0;i<W;i++){ uint64_t v=s->dom[x].w[i]&allow.w[i]; if(v!=s->dom[x].w[i]) ch=1; s->dom[x].w[i]=v; any|=v; }
  if(!any) return -1; return ch;
}
static int remove_tile(St *s,int x,int t,int *q,int *qn,unsigned char *inq){
  int dead=0;
  for(int z=0;z<NC;z++) if(z!=x){
    if(!bs_and_nz(&s->dom[z],&tilemask[t])) continue;
    for(int i=0;i<W;i++) s->dom[z].w[i]&=~tilemask[t].w[i];
    if(bs_empty(&s->dom[z])) dead=1;
    if(q && !inq[z]){ inq[z]=1; q[(*qn)++]=z; }
  }
  return dead;
}
/* propagate after dom[x] was narrowed (cells in q). returns 1 alive, 0 dead */
static int prop_ac(St *s,int *q,int qn,unsigned char *inq){
  int qh=0; static int Q[1<<16]; int m=0; for(int i=0;i<qn;i++) Q[m++]=q[i];
  while(qh<m){
    int y=Q[qh++]; inq[y]=0;
    if(qh>60000){ /* compact */ memmove(Q,Q+qh,(m-qh)*sizeof(int)); m-=qh; qh=0; }
    if(!s->fix[y] && bs_cnt(&s->dom[y])==1){
      s->fix[y]=1; int t=bs_first(&s->dom[y])>>2;
      if(remove_tile(s,y,t,Q,&m,inq)) return 0;
    }
    for(int d=0;d<4;d++){ int x=nb[y][d]; if(x<0) continue;
      int r=revise(s,x,(d+2)&3); if(r<0) return 0;
      if(r>0 && !inq[x]){ inq[x]=1; Q[m++]=x; } }
  }
  return 1;
}
static int assign(St *s,int x,int v){
  memset(&s->dom[x],0,sizeof(bs)); s->dom[x].w[v>>6]=1ULL<<(v&63);
  if(!s->dec[x]){ s->dec[x]=1; s->ndec++; }
  if(PROP==AC){ unsigned char inq[MAXC]; memset(inq,0,sizeof inq); int q[1]={x}; inq[x]=1; return prop_ac(s,q,1,inq); }
  s->fix[x]=1; int dead=remove_tile(s,x,v>>2,0,0,0);
  for(int d=0;d<4;d++){ int y=nb[x][d]; if(y<0||s->dec[y]) continue; if(revise(s,y,(d+2)&3)<0) dead=1; }
  if(PROP==FC && dead) return 0;
  return 1;
}
static void search(St *s,int depth);
static void try_assign(St *s,int x,int v,int depth){
  if(censored) return; if(++tried>cap){ censored=1; return; } prof[depth]++;
  St t=*s; if(assign(&t,x,v)) search(&t,depth+1);
}
static void search(St *s,int depth){
  if(censored) return;
  if(RULE==EDGE){
    int bx=-1,bd=0,bc=1<<30;
    for(int x=0;x<NC;x++) for(int d=1;d<=2;d++){ int y=nb[x][d]; if(y<0) continue; int c=0;
      for(int k=1;k<=NCOL;k++) if(bs_and_nz(&s->dom[x],&bycol[d][k]) && bs_and_nz(&s->dom[y],&bycol[(d+2)&3][k])) c++;
      if(c>=2 && c<bc){ bc=c; bx=x; bd=d; } }
    if(bx<0){ sols++; return; }
    int y=nb[bx][bd];
    for(int k=1;k<=NCOL;k++){
      if(!(bs_and_nz(&s->dom[bx],&bycol[bd][k]) && bs_and_nz(&s->dom[y],&bycol[(bd+2)&3][k]))) continue;
      if(++tried>cap){ censored=1; return; } prof[depth]++;
      St t=*s; for(int i=0;i<W;i++){ t.dom[bx].w[i]&=bycol[bd][k].w[i]; t.dom[y].w[i]&=bycol[(bd+2)&3][k].w[i]; }
      unsigned char inq[MAXC]; memset(inq,0,sizeof inq); int q[2]={bx,y}; inq[bx]=inq[y]=1;
      if(prop_ac(&t,q,2,inq)) search(&t,depth+1);
      if(censored) return;
    }
    return;
  }
  if(s->ndec==NC){ sols++; return; }
  int bx=-1,bc=1<<30;
  if(RULE==ROW){ for(int x=0;x<NC;x++) if(!s->dec[x]){ bx=x; break; } }
  else if(RULE==YK){
    int r0=-1; for(int x=0;x<NC;x++) if(!s->dec[x]){ r0=x/n; break; }
    for(int x=r0*n;x<NC && x/n<r0+KOPEN;x++){ if(s->dec[x]) continue;
      int r=x/n,c=x%n; if(r>0&&!s->dec[x-n]) continue; if(c>0&&!s->dec[x-1]) continue;
      int k=bs_cnt(&s->dom[x]); if(k<bc){ bc=k; bx=x; } }
  } else { for(int x=0;x<NC;x++) if(!s->dec[x]){ int k=bs_cnt(&s->dom[x]); if(k<bc){ bc=k; bx=x; } } }
  if(RULE==MIX||RULE==TILE){
    int cnt[MAXC]; unsigned char used[MAXC]; memset(cnt,0,sizeof cnt); memset(used,0,sizeof used);
    for(int x=0;x<NC;x++){ if(s->dec[x]){ used[bs_first(&s->dom[x])>>2]=1; continue; }
      for(int i=0;i<W;i++){ uint64_t b=s->dom[x].w[i]; while(b){ int v=i*64+__builtin_ctzll(b); b&=b-1; cnt[v>>2]++; } } }
    int bt=-1,btc=1<<30; for(int t=0;t<NC;t++) if(!used[t] && cnt[t]<btc){ btc=cnt[t]; bt=t; }
    if(bt>=0 && (RULE==TILE || btc<bc)){
      if(btc==0) return; /* a tile with no place left */
      for(int x=0;x<NC;x++){ if(s->dec[x]) continue;
        for(int i=0;i<W;i++){ uint64_t b=s->dom[x].w[i]&tilemask[bt].w[i]; while(b){ int v=i*64+__builtin_ctzll(b); b&=b-1; try_assign(s,x,v,depth); } } }
      return;
    }
  }
  for(int i=0;i<W;i++){ uint64_t b=s->dom[bx].w[i]; while(b){ int v=i*64+__builtin_ctzll(b); b&=b-1; try_assign(s,bx,v,depth); } }
}
static void gen(int cf,int ci,uint64_t seed){
  int t[MAXC][4];
  rs=seed*0x9E3779B97F4A7C15ULL+0x1234567ULL; for(int i=0;i<20;i++) rnd();
  for(;;){
    memset(t,0,sizeof t);
    for(int r=0;r<n;r++) for(int c=0;c<n;c++){ int x=r*n+c;
      if(c+1<n){ int fr=(r==0||r==n-1); int k=fr? 1+rnd()%cf : cf+1+rnd()%ci; t[x][1]=k; t[x+1][3]=k; }
      if(r+1<n){ int fr=(c==0||c==n-1); int k=fr? 1+rnd()%cf : cf+1+rnd()%ci; t[x][2]=k; t[x+n][0]=k; } }
    int ok=1;
    for(int a=0;a<NC&&ok;a++){
      for(int ro=1;ro<4&&ok;ro++){ int same=1; for(int d=0;d<4;d++) if(t[a][d]!=t[a][(d+ro)&3]) same=0; if(same) ok=0; }
      for(int b=a+1;b<NC&&ok;b++) for(int ro=0;ro<4&&ok;ro++){ int same=1; for(int d=0;d<4;d++) if(t[a][d]!=t[b][(d+ro)&3]) same=0; if(same) ok=0; } }
    if(ok) break;
  }
  int perm[MAXC]; for(int i=0;i<NC;i++) perm[i]=i;
  for(int i=NC-1;i>0;i--){ int j=rnd()%(i+1); int z=perm[i]; perm[i]=perm[j]; perm[j]=z; }
  for(int i=0;i<NC;i++){ int ro=rnd()&3; for(int r=0;r<4;r++) for(int d=0;d<4;d++) col[4*i+r][d]=t[perm[i]][(d-r-ro+8)&3]; }
}
int main(int argc,char**argv){
  if(argc<6){ fprintf(stderr,"usage: n cf ci seed cap [variants]\n"); return 1; }
  n=atoi(argv[1]); int cf=atoi(argv[2]), ci=atoi(argv[3]); uint64_t seed=strtoull(argv[4],0,10); cap=strtoull(argv[5],0,10);
  const char *only= argc>6? argv[6] : 0;
  NC=n*n; NV=4*NC; NCOL=cf+ci; gen(cf,ci,seed);
  for(int v=0;v<NV;v++){ tilemask[v>>2].w[v>>6]|=1ULL<<(v&63); for(int d=0;d<4;d++) bycol[d][col[v][d]].w[v>>6]|=1ULL<<(v&63); }
  St s0; memset(&s0,0,sizeof s0);
  for(int x=0;x<NC;x++){ int r=x/n,c=x%n; nb[x][0]=r>0?x-n:-1; nb[x][1]=c<n-1?x+1:-1; nb[x][2]=r<n-1?x+n:-1; nb[x][3]=c>0?x-1:-1;
    for(int v=0;v<NV;v++){ int ok=1; for(int d=0;d<4;d++) if((nb[x][d]<0)!=(col[v][d]==0)) ok=0; if(ok) s0.dom[x].w[v>>6]|=1ULL<<(v&63); } }
  struct { const char *name; int prop, rule, k; } V[]={
    {"ROW-PLAIN",PLAIN,ROW,0},{"ROW-FC",FC,ROW,0},{"Y2-FC",FC,YK,2},{"Y3-FC",FC,YK,3},{"Y5-FC",FC,YK,5},{"DOM-FC",FC,DOM,0},
    {"MIX-FC",FC,MIX,0},{"TILE-FC",FC,TILE,0},
    {"ROW-AC",AC,ROW,0},{"Y2-AC",AC,YK,2},{"Y3-AC",AC,YK,3},{"Y5-AC",AC,YK,5},{"DOM-AC",AC,DOM,0},{"MIX-AC",AC,MIX,0},{"TILE-AC",AC,TILE,0},{"EDGE-AC",AC,EDGE,0}};
  for(unsigned i=0;i<sizeof V/sizeof V[0];i++){
    if(only && !strstr(only,V[i].name)) continue;
    PROP=V[i].prop; RULE=V[i].rule; KOPEN=V[i].k; tried=0; sols=0; censored=0; memset(prof,0,sizeof prof);
    clock_t t0=clock(); St s=s0; int alive=1;
    if(PROP==AC){ unsigned char inq[MAXC]; int q[MAXC]; for(int x=0;x<NC;x++){ q[x]=x; inq[x]=1; } alive=prop_ac(&s,q,NC,inq); }
    if(alive) search(&s,0);
    int pk=0; for(int d=0;d<4*NC;d++) if(prof[d]>prof[pk]) pk=d;
    printf("{\"n\":%d,\"cf\":%d,\"ci\":%d,\"seed\":%llu,\"variant\":\"%s\",\"tried\":%llu,\"sols\":%llu,\"censored\":%d,\"peak_depth\":%d,\"peak\":%llu,\"sec\":%.2f}\n",
      n,cf,ci,(unsigned long long)seed,V[i].name,(unsigned long long)tried,(unsigned long long)sols,censored,pk,(unsigned long long)prof[pk],(double)(clock()-t0)/CLOCKS_PER_SEC);
    fflush(stdout);
  }
  return 0;
}
