import os
import subprocess
import json


def missing_test_warning(tests):
    tests_filenames = set([test['filename'] for test in tests])
    for root, _, files in os.walk('tests/dubious_snippets'):
        for os_file in files:
            if os_file.endswith('.dpl'):
                relative_path = os.path.relpath(os.path.join(
                    root, os_file), 'tests/dubious_snippets')
                if relative_path not in tests_filenames:
                    print(f"Warning: Missing test for {relative_path}")


def get_tests():
    with open('tests/test_data.json') as file:
        return json.load(file)


def compile_test(filename):
    subprocess.run(
        f"./dubious -S tests/dubious_snippets/{filename} "
        f"-o tests/dubious_asm/{filename}.s", shell=True)

    subprocess.run(
        f"./dubious tests/dubious_snippets/{filename} "
        f"-o tests/dubious_executables/{filename}.out", shell=True)
    if os.path.exists(f"tests/dubious_executables/{filename}.out"):
        return True
    print(f"Failed to compile {filename}")


if __name__ == '__main__':
    tests = get_tests()

    failed_tests = []

    for test in tests:
        print(f'Compiling {test["name"]}')
        # Ensure directories exist
        asm_dir = os.path.dirname(f"tests/dubious_asm/{test['filename']}.s")
        exec_dir = os.path.dirname(
            f"tests/dubious_executables/{test['filename']}.out")
        os.makedirs(asm_dir, exist_ok=True)
        os.makedirs(exec_dir, exist_ok=True)

        # Remove existing files if they exist
        try:
            os.remove(f"tests/dubious_asm/{test['filename']}.s")
        except FileNotFoundError:
            pass
        try:
            os.remove(f"tests/dubious_executables/{test['filename']}.out")
        except FileNotFoundError:
            pass
        compile_test(test['filename'])

    for test in tests:
        print(f'Running test {test["filename"]}')
        result = subprocess.run(
            f"tests/dubious_executables/{test['filename']}.out", shell=True)

        if result.returncode != test["output"]:
            print(f'Test {test["name"]} failed')
            failed_tests.append(
                {"name": test["name"],
                 "expected": test["output"],
                 "actual": result.returncode})
            # exit(1)

    if failed_tests:
        print(f'Failed {len(failed_tests)} tests:')
        for test in failed_tests:
            print(
                f"{test['name']}: Expected {test['expected']} "
                f"but got {test['actual']}")
        exit(1)

    missing_test_warning(tests)

    print(f'All {len(tests)} tests passed')
