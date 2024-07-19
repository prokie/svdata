class SvData:
    modules: list[SvModule]

class SvModule:
    identifier: str
    filepath: str

def read_sv_file(file_path: str) -> SvData: ...
