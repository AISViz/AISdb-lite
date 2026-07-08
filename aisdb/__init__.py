import importlib
import importlib.metadata
import logging
import os

try:
    __version__ = importlib.metadata.version("aisdb")
except importlib.metadata.PackageNotFoundError:  # running from a bare checkout
    __version__ = "0.0.0+unknown"

sqlpath = os.path.abspath(os.path.join(os.path.dirname(__file__), "aisdb_sql"))

from .database.decoder import decode_msgs

from .database.dbconn import DBConn, PostgresDBConn

from .database.dbqry import DBQuery

from .database import sqlfcn

from .database import sqlfcn_callbacks

from .gis import (
    Domain,
    DomainFromTxts,
    DomainFromPoints,
    delta_knots,
    delta_meters,
    delta_seconds,
    distance3D,
    dt_2_epoch,
    epoch_2_dt,
    radial_coordinate_boundary,
    vesseltrack_3D_dist,
)

from .interp import (
    interp_time,
)

from .receiver import start_receiver

from .proc_util import (
    glob_files,
    write_csv,
)

from .track_gen import (
    TrackGen,
    split_timedelta,
    fence_tracks,
    split_tracks,
)
from .denoising_encoder import (
    encode_score,
    encode_greatcircledistance,
    remove_pings_wrt_speed,
)

# Optional feature modules are resolved lazily (PEP 562) so that a bare
# ``import aisdb`` does not require their heavy dependencies (selenium,
# xarray/cdsapi, h3/matplotlib/geopandas, pillow/py7zr). Install the
# matching extra from pyproject.toml to use them.
_LAZY_ATTRIBUTES = {
    "Gebco": "aisdb.webdata.bathymetry",
    "ShoreDist": "aisdb.webdata.shore_dist",
    "PortDist": "aisdb.webdata.shore_dist",
    "CoastDist": "aisdb.webdata.shore_dist",
    "InlandDenoising": "aisdb.denoising_encoder",
    "graph": "aisdb.network_graph",
    "WeatherDataStore": "aisdb.weather.data_store",
    "Discretizer": "aisdb.discretize.h3",
    "WorldPortIndexClient": "aisdb.ports.api",
}


def __getattr__(name):
    if name in _LAZY_ATTRIBUTES:
        module = importlib.import_module(_LAZY_ATTRIBUTES[name])
        attribute = getattr(module, name)
        globals()[name] = attribute
        return attribute
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


def __dir__():
    return sorted(set(globals()) | set(_LAZY_ATTRIBUTES))


LOGLEVEL = os.environ.get("LOGLEVEL", "INFO")
logging.basicConfig(format="%(message)s", level=LOGLEVEL, datefmt="%Y-%m-%d %I:%M:%S")
