from typing import (
    Any,
    Callable,
)
from pathlib import Path
from functools import wraps

from errors import (
    PathError,
    pathval_err_msg,
    typecomp_err_msg,
)


def typecomp(callable_: Callable) -> Callable:
    @wraps(callable_)
    def wrapper(self: Any, other: Any):
        if not isinstance(other, type(self)):
            raise TypeError(
                typecomp_err_msg(
                    type(self).__name__,
                )
            )
        return callable_(
            self,
            other,
        )
    return wrapper


def pathval(path: Path) -> Path:
    if path.exists() and path.is_file():
        return path

    raise PathError(
        pathval_err_msg(path)
    )