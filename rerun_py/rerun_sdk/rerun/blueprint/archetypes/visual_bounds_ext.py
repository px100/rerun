from __future__ import annotations

from typing import Any

from ... import components, datatypes
from ...error_utils import catch_and_log_exceptions


class VisualBoundsExt:
    """Extension for [VisualBounds][rerun.blueprint.archetypes.VisualBounds]."""

    def __init__(
        self: Any,
        *,
        x_range: datatypes.Range1DLike,
        y_range: datatypes.Range1DLike,
    ):
        """
        Create a new instance of the VisualBounds archetype.

        Parameters
        ----------
        x_range:
            The minimum visible range of the X-axis (usually left and right bounds).
        y_range:
            The minimum visible range of the Y-axis (usually left and right bounds).

        """

        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(range2d=components.Range2D(x_range=x_range, y_range=y_range))
            return
        self.__attrs_clear__()
