import os, json

"""
Output structure:
{
    used_functions: [
        {
            index: 0,
            name: ""
        }
    ],
    variables_amount: 0,
    main_program_instructions: [
        {
            fnname: "",
            arg: {
                type: "string" | "variable" | "subprogram",
            }
        }
    ],
    subprograms: [
        [
            // Same as /main_program
        ]
    ]
}
"""

program = []

variable_to_index = {}

function_config_path = os.environ["FUNCTION_CONFIG_PATH"]
function_config = json.load(open(function_config_path))
input_program_path = os.environ["INPUT_PROGRAM_PATH"]
output_file_path = os.environ["OUTPUT_FILE_PATH"]

def process_prog(prog):
    FINDING_NAME = 0
    READING_NAME = 1
    READING_INPUT = 2
    stage = FINDING_NAME
    namebuf = ""
    inputbuf = ""
    nesting = 0
    dependencies = set()
    for c in prog:
        if stage == FINDING_NAME:
            if c == ".":
                stage = READING_NAME
        elif stage == READING_NAME:
            if c == "(":
                stage = READING_INPUT
            else:
                namebuf += c
        elif stage == READING_INPUT:
            if c == ")" and nesting == 0:
                for dependency in function_config[namebuf]["dependencies"]:
                    if dependency not in dependencies:

                namebuf = ""
                inputbuf = ""
            else:
                inputbuf += c
                if c == ")":
                    nesting -= 1
                elif c == "(":
                    nesting += 1
