from svdata import read_sv_file

ansi_module_a = read_sv_file("tests/systemverilog/ansi_module_a.sv").modules[0]


def test_module_name() -> None:
    assert ansi_module_a.identifier == "ansi_module_a"
