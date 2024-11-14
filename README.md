# taptempo

A tool for measuring the tempo in BPM of taps on keyboard/mouse. Can be useful
for quickly finding the tempo of a song, or measuring your heart rate.

## Installing

```
cargo install --git https://github.com/guyru/taptempo
```

## Usage

```
Usage: taptempo [OPTIONS]

Options:
  -s, --sample-size <SAMPLE_SIZE>  Number of samples to take for tempo calculation [default: 5]
  -t, --timeout <TIMEOUT>          Set the time in seconds to reset the computation [default: 5]
  -p, --precision <PRECISION>      Precision of the BPM output [default: 0]
      --bar-graph                  Display BPM as a moving bar graph
  -h, --help                       Print help
  -V, --version                    Print version
```

## Examples

## License

Copyright (C) 2024  Guy Rutenberg

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

## Authors
- Author: [Guy Rutenberg](https://www.guyrutenberg.com)
