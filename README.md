This is a practice program, which I am able to use for a specific purpose at work.
This has been uploaded to show my progress in learning Rust.

The Engineering department draws a circuit schematic in CAD software, and extracts a netlist (a net is a named node, and the netlist shows the components/pins connected to each node). The Drafting department uses this netlist to generate a physical PCB (printed-circuit board) in their own software, and extracts a netlist for the physical PCB. The Engineering department then compares netlists (schematic vs physical PCB) to ensure all connections are present and correct.

These comparisons used to be done by hand, but it is much faster and efficient to have a program do this task.

The netlists are in a similar format, but with subtle differences. This software receives the file names as arguments, extracts the data for each type (schematic vs physical, or "drawing"), and performs comparisons between the data. Any nets or component connections that do not match are printed out to the screen, and can be scrutinized on a case-by-case basis.

There are two sets of sample files.
The "schematic_good.txt" and "drawing_good.txt" files are a correct match, which shows the output when the files match.
The "schematic_bad.txt" and "drawing_bad.txt" files contain mis-matches, which shows the output when the files do not match.
