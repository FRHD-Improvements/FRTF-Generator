# FRTF Generator

This is a rust program that utilizes the [frhd.rs](https://github.com/FRHD-Improvements/FRHD.rs) library to create ridable tracks from the FRTF track format.

## Usage

**NOTE: This `Usage` section replaces the old `spec.md` file. Please notice that certain things have changed.**

Using a Terminal/CMD, use the binary with a `.frtf` file, like so:
`frtf track.frtf`

Here are the available instructions, each seperated by newlines.


Create a black line starting at (0, 0) and ending at (100, 100): `p 0 0 100 100`

Create a gray line starting at (-100, -100) and ending at (0, 0): `s -100 -100 0 0`

Create a checkpoint at (30, 30): `c 30 30`

Create a star at (30, 0): `t 30 0`

Create a slow motion at (30, -30): `m 30 -30`

Create a bomb at (0, -30): `o 0 -30`

Create a gravity at (-30, -30) with a rotation of 0 (facing up): `g -30 -30 0`

Create a boost at (-30, 0) with a rotation of 180 (facing down): `b -30 0 180`
