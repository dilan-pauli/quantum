# Genome Assembly QUBO — visualizing quantum annealing

A tiny Rust program that models **DNA fragment assembly** as a **QUBO**
(Quadratic Unconstrained Binary Optimization) — the kind of problem a quantum
annealer (D-Wave) or QAOA circuit solves. The point of this project is *not*
the assembly result; it's to **watch the QUBO matrix take shape** so the
math behind quantum optimization becomes concrete.

## The model

3 fragments × 3 slots = 9 binary variables `x_i`, where `i = fragment * 3 + slot`
and `x_i = 1` means "this fragment goes in this slot".

A QUBO encodes the problem as an energy function to minimize:

```
H = Σ_i a_i·x_i  +  Σ_{i<j} b_ij·x_i·x_j
     ^ linear        ^ quadratic
     (diagonal)      (off-diagonal)
```

The 9×9 matrix `Q` holds exactly those two kinds of terms:

| matrix region | value | meaning |
|---|---|---|
| diagonal `Q[i][i]` | `−2·penalty` | linear bias: each `x_i` costs energy to turn on |
| same slot, `i≠j` | `+2·penalty` | quadratic repulsion: two fragments can't share a slot |
| adjacent slots with overlap | `−overlap` | quadratic attraction: overlapping fragments should be neighbors |
| everything else | `0` | no direct interaction |

Minimizing `H` finds the assignment (fragment order) with maximum overlap and
no slot conflicts. **Red = penalty/repulsion, blue = overlap reward/attraction,
gray = zero** in the heatmap.

## What we set up (the debugging tooling)

This repo (`quantum/genome_example`):

| file | change |
|---|---|
| `genome_test/src/main.rs` | fixed two compile errors (`n` → `num_of_fragments`, `fragments` → `fragmets`); added `print_qubo()` heatmap renderer |
| `devenv.nix` | added `pkgs.lldb` (the `lldb` CLI **and** the `lldb-dap` debug adapter) |
| `.lldb/qubo_matrix.py` | **new** — LLDB summary that renders a `Vec<Vec<f64>>` as a grid; self-registers on import |

Your nvim dotfiles (`~/server-nixos`):

| file | change |
|---|---|
| `shared/neovim.nix` | added `lldb` to `extraPackages` so nvim always finds `lldb-dap` |
| `shared/neovim/lua/plugins/dap.lua` | **new** — `lldb-dap` adapter + Rust launch configuration; discovers and imports every `.lldb/*.py` script (no hardcoded names) |

> LazyVim's `dap.core` extra was already enabled, so `nvim-dap`, `nvim-dap-ui`
> and `nvim-dap-virtual-text` were already installed — only the Rust adapter and
> debugger binary were missing.

## How to learn with it

### Prerequisites (one-time)

1. Rebuild your config so the new nvim plugin + `lldb` land:
   ```bash
   cd ~/server-nixos
   nh os switch .          # or: nh os test .   (test generation, reverts on reboot)
   ```
2. Refresh devenv so `lldb` is available in the shell (direnv usually does this
   automatically; otherwise):
   ```bash
   cd ~/quantum/genome_example
   devenv shell
   ```

### 1. Run it — see the final matrix

```bash
cd ~/quantum/genome_example/genome_test
cargo run
```

You get the 9×9 QUBO as a colored grid plus two spot-checks. The diagonal is
`−100`, same-slot collisions are `+100`, and the overlap rewards are `−4`
(A→B), `−2` (A→C), `−1` (B→A), `−4` (B→C) in their cells.

### 2. Step through it in nvim — watch the matrix build

1. `cd` into the project (crate root or `genome_example`) and run `nvim`.
2. Open `genome_test/src/main.rs`.
3. Set a breakpoint with `<leader>db` (default leader is `<space>`) inside
   `build_qubo_matrix`.
4. `<leader>dc` (continue) — nvim asks which configuration; pick
   **`debug (lldb-dap)`**.
5. `<leader>du` to open the DAP UI (variables / stack / scopes / REPL).
6. In the **Variables** panel, `qubo` renders as a grid (courtesy of
   `qubo_matrix.py`) instead of 81 nested leaves.

Useful breakpoints for watching the matrix fill in:

| breakpoint | what `qubo` shows |
|---|---|
| after the overlap loop (≈ line 43) | only blue off-diagonal cells: the overlap rewards |
| after the slot-penalty loop (≈ line 58) | diagonal `−50` appears; `+100` between fragments in the same slot |
| after the fragment-penalty loop (≈ line 72) | diagonal drops to `−100`; `+100` between slots of the same fragment — the full QUBO |

### Keybindings (LazyVim DAP)

| key | action |
|---|---|
| `<leader>db` | toggle breakpoint |
| `<leader>dc` | run / continue |
| `<leader>dO` | step over |
| `<leader>di` | step into |
| `<leader>do` | step out |
| `<leader>dC` | run to cursor |
| `<leader>dt` | terminate |
| `<leader>du` | toggle DAP UI |
| `<leader>de` | evaluate expression under cursor / selection |
| `<leader>dr` | toggle REPL |
| `<leader>dl` | run last configuration |

### 3. Manual LLDB (no editor)

```bash
cd ~/quantum/genome_example/genome_test
devenv shell
rust-lldb target/debug/genome_test
```

Then inside lldb:

```
(lldb) breakpoint set --file main.rs --line 134
(lldb) run
(lldb) frame variable qubo
```

`rust-lldb` auto-loads the Rust pretty-printers; `frame variable qubo` prints
the full grid. This is exactly what `dap.lua` feeds the adapter via
`initCommands`, so you can debug the visualization logic itself here.

## How the pretty-printing works

`dap.lua` passes commands to `lldb-dap` at launch (via `initCommands`):

1. `command script import <sysroot>/lib/rustlib/etc/lldb_lookup.py` — the Rust
   std formatters shipped with the toolchain (found at runtime via
   `rustc --print sysroot`), so `Vec` shows its elements.
2. For every `.lldb/*.py` it finds walking up from the crate (nearest first),
   `command script import <script>` — no script names are hardcoded in the
   nvim config.

Each script registers itself: `command script import` calls the script's
`__lldb_init_module`, which runs its `type summary add ...` (see
`.lldb/qubo_matrix.py`). Adding a new helper is just dropping a `*.py` file
into a project `.lldb/` directory — the nvim config never changes.

The summary only affects the *display*; the nested children remain expandable,
so you can still drill into a single cell.
