"""
This library is the python bindings of the image_to_console_core project
"""
from .image_to_console_core_python import *

__doc__ = image_to_console_core_python.__doc__
if hasattr(image_to_console_core_python, "__all__"):
    __all__ = image_to_console_core_python.__all__