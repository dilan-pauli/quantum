"""LLDB formatter: render a Rust `Vec<Vec<f64>>` (a QUBO matrix) as a grid.

A QUBO matrix `Q` is square; row/column `i` is the binary variable `x_i`.
This summary flattens the nested `Vec<Vec<f64>>` into one `[a b c ...]` line
per row so you can read the whole matrix at a glance in the debugger's
variables view instead of expanding 81 leaves by hand.

Requires the Rust std formatters to be imported FIRST, so the outer `Vec`
exposes each inner `Vec` as a child and each `f64` as a child of that:

    command script import "<sysroot>/lib/rustlib/etc/lldb_lookup.py"

Self-registering: importing this file (`command script import`) calls
`__lldb_init_module`, which installs the `type summary` for nested
`Vec<Vec<...>>`. Nothing hardcodes a script name — the debugger config just
imports whatever `*.py` files it discovers in a project's `.lldb/` directory.
"""

import lldb


def qubo_summary(valobj, internal_dict):
    """Return the matrix as a multi-line grid: one `[a b c]` line per row."""
    nrows = valobj.GetNumChildren()
    if nrows == 0:
        return "0x0"

    lines = []
    ncols = 0
    for i in range(nrows):
        row = valobj.GetChildAtIndex(i)
        ncols = row.GetNumChildren()
        cells = []
        for j in range(ncols):
            value = row.GetChildAtIndex(j).GetValue()
            cells.append(value if value is not None else "?")
        lines.append("[" + " ".join(cells) + "]")

    return "{}x{}\n{}".format(nrows, ncols, "\n".join(lines))


def __lldb_init_module(debugger, internal_dict):
    """Register the summary on import; `__name__` keeps it rename-proof."""
    debugger.HandleCommand(
        "type summary add -F {0}.qubo_summary "
        '-x "^alloc::vec::Vec<alloc::vec::Vec<.*>$"'.format(__name__)
    )
