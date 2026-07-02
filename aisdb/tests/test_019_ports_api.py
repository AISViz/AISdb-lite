"""Offline tests for the World Port Index client (network is monkeypatched)."""

import pandas as pd
import pytest

from aisdb.ports.api import WorldPortIndexClient

GEOJSON_FIXTURE = {
    "features": [
        {
            "properties": {"PORT_NAME": "HALIFAX", "CARGODEPTH": "A"},
            "geometry": {"coordinates": [-63.57, 44.65]},
        },
        {
            "properties": {"PORT_NAME": "SHEET HARBOUR", "CARGODEPTH": "Z"},
            "geometry": {"coordinates": [-62.53, 44.92]},
        },
    ]
}


class _FakeResponse:
    def raise_for_status(self):
        pass

    def json(self):
        return GEOJSON_FIXTURE


def test_fetch_ports_parses_features(monkeypatch):
    captured = {}

    def fake_get(url, params=None, timeout=None):
        captured["url"] = url
        captured["params"] = params
        captured["timeout"] = timeout
        return _FakeResponse()

    monkeypatch.setattr("requests.get", fake_get)
    client = WorldPortIndexClient()
    df = client.fetch_ports(lat_min=43.0, lat_max=46.0, lon_min=-65.0, lon_max=-60.0)

    assert len(df) == 2
    assert set(df["PORT_NAME"]) == {"HALIFAX", "SHEET HARBOUR"}
    assert df.loc[df["PORT_NAME"] == "HALIFAX", "LAT"].iloc[0] == 44.65
    assert df.loc[df["PORT_NAME"] == "HALIFAX", "LON"].iloc[0] == -63.57
    assert captured["timeout"] is not None  # request must not hang forever
    assert "LATITUDE >= 43.0" in captured["params"]["where"]


def test_fetch_ports_save_requires_out_path(monkeypatch):
    monkeypatch.setattr("requests.get", lambda *a, **k: _FakeResponse())
    client = WorldPortIndexClient()
    with pytest.raises(ValueError):
        client.fetch_ports(43.0, 46.0, -65.0, -60.0, save=True)


@pytest.mark.parametrize(
    "bounds",
    [
        (46.0, 43.0, -65.0, -60.0),  # lat_min > lat_max
        (43.0, 46.0, -60.0, -65.0),  # lon_min > lon_max
        (43.0, 95.0, -65.0, -60.0),  # latitude out of domain
        (43.0, 46.0, -200.0, -60.0),  # longitude out of domain
    ],
)
def test_fetch_ports_rejects_invalid_bounds(bounds):
    client = WorldPortIndexClient()
    with pytest.raises(ValueError):
        client._build_where_clause(*bounds)


def test_where_clause_rejects_non_numeric():
    client = WorldPortIndexClient()
    with pytest.raises(TypeError):
        client._build_where_clause("43; DROP TABLE ports", 46.0, -65.0, -60.0)


def test_filter_by_cargo_depth():
    client = WorldPortIndexClient()
    df = pd.DataFrame({"PORT_NAME": ["A", "B", "C"], "CARGODEPTH": ["A", "Z", "F"]})
    filtered = client.filter_by_cargo_depth(df)
    assert list(filtered["PORT_NAME"]) == ["A", "C"]
