# Cache layout for a pair layer

This Windows C++ benchmark measures how the order of right-hand addresses affects a layer comparing disjoint keys in the two halves of an array. Run run_native.cmd with an MSVC x64 installation. Set VCVARS64 when using another installation path. Run `python check_layouts.py` in this directory for the exhaustive small-layout check and timing summaries. The CSV files preserve the earlier measurements for replay.

With 65,536 keys, the archived measurements give median per-array layer times of 16.20 microseconds for a forward scan, 17.55 for reverse order and 27.20 for shuffled order (ten paired trials). These are layer timings on the measured machine.
