from abc import (
    ABC,
    abstractmethod,
)
from pathlib import Path

from abjet import File
from memory import HandMemory
from validation import pathval


class Action(ABC):
    def __init__(
        self,
        *files: File,
        handdb: HandMemory,
        dest: Path | None = None,
        filter_: str | None = None,
    ) -> None:
        self._files = files
        self._handdb = handdb
        self._filter = filter_
        self._dest = dest and pathval(dest)

    @property
    def files(self) -> tuple[File]:
        return self._files

    @property
    def handdb(self) -> HandMemory:
        return self._handdb

    @property
    def dest(self) -> Path | None:
        return self._dest

    @property
    def filter(self) -> str:
        return self._filter

    @abstractmethod
    def execute(self) -> None:
        pass

    @abstractmethod
    def opposite(self) -> None:
        pass

    def __repr__(self) -> str:
        return (
            f"{self.__class__.__name__}("
            f"files={self.files}, "
            f"dest='{self.dest}', "
            f"filter='{self.filter}')"
        )


class GrabAction(Action):
    ...


class DropAction(Action):
    ...


class MoveAction(Action):
    ...


class ThrowAction(Action):
    ...


class WashAction(Action):
    ...


class MakeDirAction(Action):
    ...


class TidyAction(Action):
    ...