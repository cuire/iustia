import typing
from typing import TypeVar

from _typeshed import Incomplete

read_config: Incomplete
DEFAULT_ENCODING: str
TRUE_VALUES: Incomplete
FALSE_VALUES: Incomplete

def strtobool(value): ...

class UndefinedValueError(Exception): ...
class Undefined: ...

undefined: Incomplete

class Config:
    repository: Incomplete
    def __init__(self, repository) -> None: ...
    def get(self, option, default=..., cast=...): ...
    def __call__(self, *args, **kwargs): ...

class RepositoryEmpty:
    def __init__(self, source: str = ..., encoding=...) -> None: ...
    def __contains__(self, key) -> bool: ...
    def __getitem__(self, key) -> None: ...

class RepositoryIni(RepositoryEmpty):
    SECTION: str
    parser: Incomplete
    def __init__(self, source, encoding=...) -> None: ...
    def __contains__(self, key) -> bool: ...
    def __getitem__(self, key): ...

class RepositoryEnv(RepositoryEmpty):
    data: Incomplete
    def __init__(self, source, encoding=...) -> None: ...
    def __contains__(self, key) -> bool: ...
    def __getitem__(self, key): ...

class RepositorySecret(RepositoryEmpty):
    data: Incomplete
    def __init__(self, source: str = ...) -> None: ...
    def __contains__(self, key) -> bool: ...
    def __getitem__(self, key): ...

T = TypeVar("T")
P = TypeVar("P")
R = TypeVar("R")

class AutoConfig:
    SUPPORTED: Incomplete
    encoding = str
    search_path: str
    config: Incomplete

    def __init__(self, search_path: Incomplete | None = None) -> None: ...
    @typing.overload
    def __call__(
        self,
        name: str,
        default: str | None = ...,
        cast: None = None,
    ) -> str: ...
    @typing.overload
    def __call__(
        self,
        name: str,
        default: P | None = ...,
        cast: typing.Callable[[P], R] = ...,
    ) -> R: ...

config: AutoConfig

class Csv:
    cast: Incomplete
    delimiter: Incomplete
    strip: Incomplete
    post_process: Incomplete

    def __init__(
        self,
        cast: typing.Callable[[P], R] | None = ...,
        delimiter: str = ...,
        strip: str = ...,
        post_process=...,
    ) -> None: ...
    def __call__(self, value: str) -> list[str]: ...

class Choices:
    flat: Incomplete
    cast: Incomplete
    choices: Incomplete
    def __init__(
        self, flat: Incomplete | None = ..., cast=..., choices: Incomplete | None = ...
    ) -> None: ...
    def __call__(self, value): ...
