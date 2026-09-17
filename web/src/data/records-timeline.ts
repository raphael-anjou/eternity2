// The Eternity II community record timeline — the single source of truth for
// both the records table (RecordsView) and the record-climb chart
// (RecordTimeline). Board-level verified against the groups.io archive; every
// entry links a public source where one exists.

export type RecordRow = {
  /** ISO date or partial date (YYYY, YYYY-MM, YYYY-MM-DD). */
  date: string;
  /** Score as displayed — a number, "—" for milestones, or a quoted variant. */
  score: string;
  author: string;
  /** "canonical" = the real puzzle; "variant" = a non-canonical piece set. */
  canonical: "canonical" | "variant";
  method: string;
  /** id into the viewer's board params when a real board is bundled. */
  board?: string;
  /** Public source (groups.io msg_num, Wikipedia, etc.), if any. */
  source?: { href: string; label: string };
  /** Which record line this row belongs to. Omitted means the open
   *  (starter-only) line, which is every record board from 467 up; "strict" is
   *  the all-five-clues line. The chart splits its two tracks on this field —
   *  it used to match on author name, which silently mis-tracked a row as soon
   *  as an author string changed. */
  regime?: "strict";
  /** Marks the standing strict-five-clue record, so RecordsView can pull it
   *  without matching on a board id — the 465 line came out of a distributed
   *  fleet and has many boards rather than one bundled artifact. */
  strictRecord?: true;
};

const GROUPS_IO = "https://groups.io/g/eternity2";
const WIKIPEDIA_E2 = "https://en.wikipedia.org/wiki/Eternity_II_puzzle";

export const RECORDS: RecordRow[] = [
  { date: "2007-07-28", score: "—", author: "TOMY / Christopher Monckton", canonical: "canonical", method: "Puzzle released, with a $2M prize for the first complete solution", source: { href: WIKIPEDIA_E2, label: "Wikipedia" } },
  { date: "2008-09", score: "467", author: "Louis Verhaard", canonical: "canonical", method: "Set-composition swap-annealing; won the $10,000 best-partial-solution prize", board: "Louis_Verhaard_467", source: { href: "https://www.shortestpath.se/eii/eii_details.html", label: "shortestpath.se" } },
  { date: "2010-12-31", score: "—", author: "—", canonical: "canonical", method: "The competition closes at noon; the $2M prize expires unclaimed", source: { href: WIKIPEDIA_E2, label: "Wikipedia" } },
  { date: "2020-08-31", score: "468", author: "Joshua Blackwood", canonical: "canonical", method: "Blackwood's solver (pre-release); relayed from Reddit (#10032), board verified and shared by Jef Bucas", board: "Joshua_Blackwood_468", source: { href: `${GROUPS_IO}/message/10033`, label: "groups.io #10033" } },
  { date: "2020-09-09", score: "469", author: "Peter McGavin", canonical: "canonical", method: "Blackwood's solver — the community ceiling until 2021 (“New record score of 469! Only 11 breaks!”)", board: "JBlackwood+PMcGavin_469", source: { href: `${GROUPS_IO}/message/10045`, label: "groups.io #10045" } },
  { date: "2020-11", score: "469", author: "various (~7 boards)", canonical: "canonical", method: "Blackwood's solver (Bucas's C rewrite), independent finds — plus one single-piece swap of McGavin's board", board: "JBlackwood+Jef_469_c", source: { href: `${GROUPS_IO}/message/10067`, label: "groups.io #10067" } },
  { date: "2021-03-30", score: "470", author: "Joshua Blackwood", canonical: "canonical", method: "Blackwood's solver, retuned schedule (break indexes 11→10) — same starter-only regime as the 468/469 (verified, #10554). His own account: about a month on a home Threadripper 3970X (Discord, 2024-11)", board: "Joshua_Blackwood_470", source: { href: `${GROUPS_IO}/message/10117`, label: "groups.io #10117" } },
  { date: "2023-03-09", score: "460", author: "Bruno Gauthier", canonical: "canonical", method: "Strict all-5-clue discipline — the best board respecting the four optional clues for over three years, until 2026", source: { href: `${GROUPS_IO}/message/11074`, label: "groups.io #11074" }, regime: "strict" },
  { date: "2023-10", score: "“480”", author: "various", canonical: "variant", method: "Mixed Clue-1 + Clue-2 piece sets — NOT the canonical puzzle", source: { href: `${GROUPS_IO}/message/11169`, label: "groups.io #11169" } },
  { date: "2024-12-02", score: "470", author: "Jef Bucas", canonical: "canonical", method: "Restarted threads of Blackwood's solver — another 470 tie; Carlos Fernandez posted border-rearrangement variations", board: "JBlackwood+Jef_470", source: { href: `${GROUPS_IO}/message/11401`, label: "groups.io #11401" } },
  { date: "2026-07-06", score: "464", author: "Benjamin Riotte", canonical: "canonical", method: "Strict-five-clue record (16 broken edges), all five clues at their official cells — his own modified-Blackwood DFS. Beats Gauthier's 460, unbeaten since 2023; Igor Pejic reached the same 463–464 range independently in the same thread. Superseded by the 465 within two months", board: "Benjamin_Riotte_464", source: { href: `${GROUPS_IO}/message/11919`, label: "groups.io #11919" }, regime: "strict" },
  { date: "2026-08", score: "465", author: "Benjamin Riotte & the five-clue fleet", canonical: "canonical", method: "The standing strict-five-clue record (15 broken edges), all five clues at their official cells — found by the community's shared solver fleet, which reported 11 distinct 465 boards when it opened to contributors on 2026-08-30 and many more since, credited to several contributors' machines. Carlos Fernandez's exchange analysis shows the first two belong to one local family, so the board count runs well ahead of the count of independent finds", source: { href: `${GROUPS_IO}/topic/120997660`, label: "groups.io topic 120997660" }, regime: "strict", strictRecord: true },
];
