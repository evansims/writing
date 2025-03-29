import os
import sys

sys.path.append(os.path.dirname(os.path.realpath(__file__)))

from .content import content_bp
from .audio import audio_bp

__all__ = ["content_bp", "audio_bp"]
