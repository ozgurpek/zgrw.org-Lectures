"""Subject and observers (classes.h + classes.cc in the C++ version)."""

from __future__ import annotations

from abc import ABC, abstractmethod


class Subject:
    def __init__(self) -> None:
        self._val = 1
        self._observers: list[Observer] = []

    def set_val(self, v: int) -> None:
        self._val = v
        self._notify()

    def get_val(self) -> int:
        return self._val

    def subscribe(self, o: Observer) -> None:
        self._observers.append(o)

    def _notify(self) -> None:
        for o in self._observers:
            o.update()


class Observer(ABC):
    def __init__(self, model: Subject, denom: int) -> None:
        self._model = model
        self._denom = denom
        model.subscribe(self)

    @abstractmethod
    def update(self) -> None:  # pure virtual in C++
        ...

    def get_subject(self) -> Subject:
        return self._model


class DivObserver(Observer):
    def update(self) -> None:
        v = self.get_subject().get_val()
        # // is integer division like C++ int / int (they differ only for negatives)
        print(f"{v} / {self._denom} = {v // self._denom}")


class ModObserver(Observer):
    def update(self) -> None:
        v = self.get_subject().get_val()
        print(f"{v} % {self._denom} = {v % self._denom}")
