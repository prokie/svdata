from svdata import read_sv_file

ansi_module_a = read_sv_file("tests/systemverilog/ansi_module_a.sv").modules[0]


print(ansi_module_a)
