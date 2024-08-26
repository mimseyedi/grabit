from typing import (
    Any,
    Iterator,
)
from pathlib import Path
from hashlib import sha256
from functools import reduce

from abjet import File
from memory import (
    HandMemory,
    StackMemory,
)


class Hand:
    def __init__(
        self,
        name: str,
        password: str | None = None,
        *,
        size: int | None = None,
        capacity: int = 50,
    ) -> None:
        self._name       = name
        self._size       = size
        self._capacity   = capacity
        self._password   = password and \
                           self._hash(password)

        self._handdir    = Path(
            Path(__file__).parent,
            'hands',
            self._name,
        )
        self._filesdir   = Path(
            self._handdir,
            'files',
        )
        self._settings   = Path(
            self._handdir,
            '.settings.json'
        )
        self._undomemory = StackMemory(
            Path(
                self._handdir,
                '.undomem.sm',
            )
        )
        self._redomemory = StackMemory(
            Path(
                self._handdir,
                '.redomem.sm'
            )
        )
        self._handdb = HandMemory(self)
        self._auton  = 1

    @property
    def name(self) -> str:
        return self._name

    @property
    def size(self) -> int:
        return self._size

    @size.setter
    def size(self, nsize: int) -> None:
        self._size = nsize

    @property
    def capacity(self) -> int:
        return self._capacity

    @capacity.setter
    def capacity(self, ncap: int) -> None:
        self._size = ncap

    @property
    def password(self) -> str:
        return self._password

    @password.setter
    def password(self, npass: str) -> None:
        self._password = self._hash(npass)

    @property
    def handdir(self) -> Path:
        return self._handdir

    @property
    def filesdir(self) -> Path:
        return self._filesdir

    @property
    def settings(self) -> Path:
        return self._settings

    @property
    def undomemory(self) -> StackMemory:
        return self._undomemory

    @property
    def redomemory(self) -> StackMemory:
        return self._redomemory

    @property
    def handdb(self) -> HandMemory:
        return self._handdb

    @property
    def auton(self) -> int:
        return self._auton

    def build(self) -> None:
        ...

    def dismiss(self) -> None:
        ...

    def repair(self) -> None:
        ...

    def open(self) -> None:
        ...

    def grab(self, *args, **kwargs) -> None:
        ...

    def drop(self, *args, **kwargs) -> None:
        ...

    def move(self, *args, **kwargs) -> None:
        ...

    def throw(self, *args, **kwargs) -> None:
        ...

    def wash(self, *args, **kwargs) -> None:
        ...

    def tidy(self, *args, **kwargs) -> None:
        ...

    @staticmethod
    def _hash(string: str) -> str:
        return sha256(
            string.encode('utf-8')
        ).hexdigest()

    def __iter__(self) -> Iterator:
        yield from self._handdb

    def __len__(self) -> int:
        return reduce(
            lambda c, _: c + 1, self, 0
        )

    def __add__(self, other):
        ...

    def __repr__(self) -> str:
        ...