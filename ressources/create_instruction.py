import json

# Open a file: file
file = open('instruction_set.json',mode='r')

out_file = open('tmp.rs',mode='w')

# read all lines at once
raw_data = file.read()

# close the file
file.close()

instructions = json.loads(raw_data)

mnemonics = []

out_file.writelines("UNPREFIXED\n\n\n\n")

for i in instructions["unprefixed"]:
    mnemonic = instructions["unprefixed"][i]["mnemonic"]
    length = instructions["unprefixed"][i]["bytes"]
    cycle1 = instructions["unprefixed"][i]["cycles"][0]
    cycle2 = instructions["unprefixed"][i]["cycles"][0]

    if len(instructions["unprefixed"][i]["cycles"]) > 1:
        cycle2 = instructions["unprefixed"][i]["cycles"][1]

    mnemonics.append(mnemonic)

    out_file.writelines("Instruction {\n")
    out_file.writelines(f"    opcode: {i},\n")
    out_file.writelines(f"    name: InstructionCode::{mnemonic},\n")
    out_file.writelines(f"    length: {length},\n")
    out_file.writelines(f"    cycles: [{cycle1}, {cycle2}],\n")
    out_file.writelines(f"    operation: |_cpu| 0,\n")
    out_file.writelines("},\n")

out_file.writelines("\n\n\n\n\nPREFIXED\n\n\n\n")

for i in instructions["cbprefixed"]:
    mnemonic = instructions["cbprefixed"][i]["mnemonic"]
    length = instructions["cbprefixed"][i]["bytes"]
    cycle1 = instructions["cbprefixed"][i]["cycles"][0]
    cycle2 = instructions["cbprefixed"][i]["cycles"][0]

    if len(instructions["cbprefixed"][i]["cycles"]) > 1:
        cycle2 = instructions["cbprefixed"][i]["cycles"][1]

    mnemonics.append(mnemonic)

    out_file.writelines("Instruction {\n")
    out_file.writelines(f"    opcode: {i},\n")
    out_file.writelines(f"    name: InstructionCode::{mnemonic},\n")
    out_file.writelines(f"    length: {length},\n")
    out_file.writelines(f"    cycles: [{cycle1}, {cycle2}],\n")
    out_file.writelines(f"    operation: |_cpu| 0,\n")
    out_file.writelines("},\n")


mnemonics = list(dict.fromkeys(mnemonics))
for i in mnemonics:
    out_file.writelines(f"    {i},\n")
