# Commands Block

> This feature is `Experimental`

## Command Usage Documentation

### Purpose

The provided command `index.py` is used to perform a specific action or operation. In this case, it appears to be referencing a Python script named `index.py`.

### Usage

To use this command, follow the syntax:

```
commands {
    index.py
}
```

Replace `index.py` with the actual name of the Python script you want to execute.

### Example

Suppose you have a Python script named `my_script.py` and you want to execute it using this command. Your configuration file would look like this:

```
commands {
    my_script.py
}
```

### Scripts files locations in Operations Systems:

| System  | Location                                                               |
| ------- | ---------------------------------------------------------------------- |
| Linux   | `home/<YOUR_USERNAME>/.config/scimon/scripts/`                       |
| MacOS   | `/Users/<YOUR_USERNAME>/Library/Application Support/scimon/scripts/` |
| Windows | `C:\Users\<YOUR_USERNAME>\AppData\Roaming\scimon\scripts\`           |

### Notes

- Ensure that the Python script file (`index.py` in this case) exists in the current directory or provide the full path to the script.
- Make sure you have Python installed on your system and it is accessible from the command line.
- Only Python and JavaScript are supported.
