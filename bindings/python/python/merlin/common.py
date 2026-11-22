#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Shared native error categories and one-way cooperative cancellation."""
from ._native import ErrorCode, Project, RunControl, __version__

__all__ = ["ErrorCode", "RunControl", "Project"]
