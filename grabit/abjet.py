from pathlib import Path

from validation import (
    pathval,
    typecomp,
)


class File:
    def __init__(
        self,
        id_: int,
        path: Path,
        *,
        mpath: Path | None = None
    ) -> None:
        self._id    = id_
        self._path  = pathval(path)
        self._mpath = mpath and pathval(mpath)

    @property
    def id(self) -> int:
        return self._id

    @property
    def path(self) -> Path:
        return self._path

    @property
    def mirrorpath(self) -> Path | None:
        return self._mpath

    @property
    def realpath(self) -> Path:
        return self._mpath or self._path

    @property
    def size(self) -> int:
        return self.realpath.stat().st_size

    @property
    def name(self) -> str:
        return self._path.stem

    @property
    def suffix(self) -> str:
        return self._path.suffix

    @typecomp
    def __eq__(self, other) -> bool:
        return self.size == other.size

    @typecomp
    def __ne__(self, other) -> bool:
        return self.size != other.size

    @typecomp
    def __lt__(self, other) -> bool:
        return self.size < other.size

    @typecomp
    def __gt__(self, other) -> bool:
        return self.size > other.size

    @typecomp
    def __le__(self, other) -> bool:
        return self.size <= other.size

    @typecomp
    def __ge__(self, other) -> bool:
        return self.size >= other.size

    def __repr__(self) -> str:
        return (
            f"File("
            f"id={self.id}, "
            f"path='{self.path}', "
            f"mirrorpath='{self.mirrorpath}', "
            f"realpath='{self.realpath}', "
            f"size={self.size})"
        )