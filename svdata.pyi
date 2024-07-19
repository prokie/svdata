from enum import Enum

class SvVariable:
    identifier: str

class SvPortDirection(Enum):
    Inout = "Inout"
    Input = "Input"
    Output = "Output"
    Ref = "Ref"
    IMPLICIT = "IMPLICIT"

class SvData:
    modules: list[SvModule]

class SvPort:
    identifier: str
    direction: SvPortDirection

class SvModule:
    identifier: str
    filepath: str
    ports: list[SvPort]
    variables: list[SvVariable]

def read_sv_file(file_path: str) -> SvData: ...
