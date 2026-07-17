# FASTSim Vehicle Database
This repository houses the publicly available collection of vehicle models for NLR's FASTSim on-road vehicle powertrain simulation software. Additional information on FASTSim can be found at the [FASTSim GitHub repository](https://github.com/NatLabRockies/fastsim) or on the [NLR.gov website](https://www.nlr.gov/transportation/fastsim).

## Database Organizational Schema (v1)

| Level | Description | Example |
| --- | --- | --- |
| Schema version | Describes the format of subsequent organizational levels. | `v1` |
| FASTSim version | Major FASTSim version. | `fastsim-3` |
| Powertrain | Powertrain type (e.g. `conv`/`hev`/`phev`/`bev`/`fcev`) | `hev` |
| Make | Vehicle make/manufacturer name. | `toyota` |
| Model | Vehicle model name and any applicable trim information. | `camry-xle` |
| Year | Model year or year range. | `2015` |
| Variant | Description of active modeling feature set. | `thermal` |
| Revision | Model revision number for updates and corrections. | `v1` |

Example full filepath:

`v1/fastsim-3/hev/toyota/camry-xle/2015/thermal/v1.yaml`
