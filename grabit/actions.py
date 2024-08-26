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
    def __init__(
        self,
        *args,
        deepcopy: bool = False,
        **kwargs,
    ) -> None:
        super().__init__(
            *args,
            **kwargs,
        )
        self._deepcopy = deepcopy

    @property
    def deepcopy(self) -> bool:
        return self._deepcopy

    def execute(self) -> None:
        pass

    def opposite(self) -> None:
        pass


class DropAction(Action):
    def __init__(
        self,
        *args,
        replace: bool = False,
        rename: bool = False,
        **kwargs,
    ) -> None:
        super().__init__(
            *args,
            **kwargs,
        )
        self._replace = replace
        self._rename  = rename

    @property
    def replace(self) -> bool:
        return self._replace

    @property
    def rename(self) -> bool:
        return self._rename

    def execute(self) -> None:
        pass

    def opposite(self) -> None:
        pass


class MoveAction(Action):
    def __init__(
        self,
        *args,
        replace: bool = False,
        rename: bool = False,
        chpath: bool = False,
        **kwargs,
    ) -> None:
        super().__init__(
            *args,
            **kwargs,
        )
        self._replace = replace
        self._rename  = rename
        self._chpath  = chpath

    @property
    def replace(self) -> bool:
        return self._replace

    @property
    def rename(self) -> bool:
        return self._rename

    @property
    def chpath(self) -> bool:
        return self._chpath

    def execute(self) -> None:
        pass

    def opposite(self) -> None:
        pass


class ThrowAction(Action):
    def __init__(
        self,
        *args,
        **kwargs,
    ) -> None:
        super().__init__(
            *args,
            **kwargs,
        )

    def execute(self) -> None:
        pass

    def opposite(self) -> None:
        pass


class WashAction(Action):
    def __init__(
        self,
        *args,
        **kwargs,
    ) -> None:
        super().__init__(
            *args,
            **kwargs,
        )

    def execute(self) -> None:
        pass

    def opposite(self) -> None:
        pass


class TidyAction(Action):
    def __init__(
        self,
        *args,
        **kwargs,
    ) -> None:
        super().__init__(
            *args,
            **kwargs,
        )

    def execute(self) -> None:
        pass

    def opposite(self) -> None:
        pass