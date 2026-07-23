# FASTSim Vehicle Database
This repository houses the publicly available collection of vehicle models for NLR's FASTSim on-road vehicle powertrain simulation software. Additional information on FASTSim can be found at the [FASTSim GitHub repository](https://github.com/NatLabRockies/fastsim), the [FASTSim documentation site](https://natlabrockies.github.io/fastsim/), or on the [NLR.gov FASTSim homepage](https://www.nlr.gov/transportation/fastsim).

## Database Organizational Schema (v1)

| Level | Description | Example |
| --- | --- | --- |
| Schema version | Describes the format of subsequent organizational levels. | `v1` |
| FASTSim version | Major FASTSim version. | `fastsim-3` |
| Powertrain | Powertrain type (e.g. `conv`/`hev`/`phev`/`bev`/`fcev`) | `conv` |
| Make | Vehicle make/manufacturer name. | `ford` |
| Model | Vehicle model name and any applicable trim information. | `fusion` |
| Year | Model year or year range. | `2012` |
| Variant | Description of active modeling feature set. | `base` |
| Revision | Model revision number for updates and corrections. | `r1` |

Example full filepath:

`v1/fastsim-3/conv/ford/fusion/2012/base/r1.yaml`

## Loading from this Database

FASTSim provides the `Vehicle.from_db` method that loads from this database by default
(requires internet access):

```python
import fastsim

vehicle_path = "v1/fastsim-3/conv/ford/fusion/2012/base/r1"

veh = fastsim.Vehicle.from_db(path=vehicle_path)
```

Alternatively, you can supply path segments as individual fields, which is useful for programmatic searching and filtering.

```python
fastsim.Vehicle.from_db(
    fastsim_version=3,       # optional, defaults to installed FASTSim major version
    powertrain="conv",       # required: "conv", "hev", "phev", "bev"
    make="ford",             # required
    model="fusion",          # required
    year="2012",             # required (string; supports ranges like "2020-2023")
    variant="base",          # optional, default "base"
    revision=1,              # required (int or "r1"/"R1" string)
    extension="yaml",        # optional, default "yaml"
)
```

For more information on `Vehicle.from_db` and how to load from alternate databases, see the Python docstring or the [relevant page of the FASTSim documentation](https://natlabrockies.github.io/fastsim/vehicle-databases).
