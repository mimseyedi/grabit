import pickle
from typing import (
    Any,
    Iterator,
)
from pathlib import Path

from actions import Action
from validation import pathval
from errors import (
    StackMemoryError,
    emptystack_err_msg,
)


class StackMemory:
    def __init__(
        self,
        path: Path,
        size: int = 10,
    ) -> None:
        self._size = size
        self._path = pathval(path)

    @property
    def path(self):
        return self._path

    @property
    def size(self):
        return self._size

    def push(self, item: Any) -> None:
        items = self._read()

        if len(items) >= 10:
            items.pop(0)

        items.append(item)
        self._write(items)

    def pop(self) -> None:
        items = self._read()

        if len(items):
            items.pop()
            self._write(items)
            return

        raise StackMemoryError(
            emptystack_err_msg(
                self.path,
            )
        )

    def build(self):
        self.path.touch(exist_ok=True)

    def _read(self) -> list[Action]:
        with open(
            file=self.path,
            mode='rb',
        ) as file:
            stack = pickle.load(file)
        return stack

    def _write(self, items: list[Action]) -> None:
        with open(
            file=self.path,
            mode='wb',
        ) as file:
            pickle.dump(items, file)

    def __iter__(self) -> Iterator:
        yield from self._read()

    def __len__(self) -> int:
        return len(self._read())

    def __repr__(self) -> str:
        return (
           f"StackMemory("
           f"path='{self.path}', "
           f"size={self.size}, "
           f"items={list(iter(self))})"
        )

    def __str__(self) -> str:
        return (
            f"StackMemory("
            f"path='{self.path}', "
            f"size={self.size})"
        )


class HandMemory:
    def __init__(self, hand) -> None:
        self._hand = hand
        self._path = Path(
            hand.handdir,
            f'.{hand.name}.db',
        )

    @property
    def hand(self):
        return self._hand

    @property
    def path(self):
        return self._path

    def build(self) -> None:
        self.path.touch(exist_ok=True)

    def read(self) -> dict:
        with open(
            file=self.path,
            mode='rb',
        ) as dbfile:
            db = pickle.load(dbfile)
        return db

    def write(self, db) -> None:
        with open(
            file=self.path,
            mode='wb',
        ) as dbfile:
            pickle.dump(db, dbfile)

    def __iter__(self) -> Iterator:
        yield from self.read().items()

    def __len__(self) -> int:
        return len(self.read())

    def __repr__(self) -> str:
        return (
            f"HandDB("
            f"hand={self.hand.name}, "
            f"path='{self.path}')"
        )