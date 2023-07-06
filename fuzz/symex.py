#!/usr/bin/env python

# super simple PoC for running the cargo fuzz harnesses with tritondse.

import sys
import logging
import hashlib
from pathlib import Path

from tritondse import SymbolicExplorator, SymbolicExecutor, ProcessState, Seed, Config, CoverageStrategy, Program, SeedFormat, CompositeData
from tritondse import ProcessState, CleLoader


logging.basicConfig(level=logging.DEBUG)
is_debug = True

def symex_program(program_path: Path, corpus_path: Path):

    loader = CleLoader(str(program_path))

    input_file = "/tmp/tritondse-input"

    format = SeedFormat.COMPOSITE
    config = Config(coverage_strategy = CoverageStrategy.PATH,
                    debug = is_debug,
                    pipe_stdout = is_debug,
                    pipe_stderr = is_debug,
                    execution_timeout = 1,
                    program_argv = [str(program_path), str(input_file)],
                    smt_timeout= 50,
                    seed_format = format)
    # Create an instance of the Symbolic Explorator
    dse = SymbolicExplorator(config, loader)

    for seed_path in Path(corpus_path).iterdir():
        with seed_path.open("rb") as fb:
            data = fb.read()
        seed = Seed(CompositeData(files={"stdin": b"", # nothing on stdin
                                  input_file: data}))
        dse.add_input_seed(seed)

    # hashes = set()
    # def pre_exec_hook(se: SymbolicExecutor, state: ProcessState):
    #     if se.seed.hash not in hashes:
    #         hashes.add(se.seed.hash)
    #         filename = corpus_path / f"dse-{se.seed.hash}"
    #         if not os.path.exists(filename):
    #             if is_debug:
    #                 print('Creating queue input ' + filename)
    #             with open(filename, 'wb') as file:
    #                 file.write(se.seed.content.files[input_file])
    # dse.callback_manager.register_pre_execution_callback(pre_exec_hook)

    dse.explore()


def main(harness_name: str):
    program_path = "target/x86_64-unknown-linux-gnu/release/" + harness_name
    corpus_path = "./corpus/" + harness_name + "/"
    symex_program(Path(program_path), Path(corpus_path))


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(sys.argv[0], "<cargo_fuzz_harness_name")
    else:
        main(sys.argv[1])
