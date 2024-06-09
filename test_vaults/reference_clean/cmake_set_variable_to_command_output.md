---
aliases:
tags:
  - cmake
  - coding
bad_links:
---
# CMake Set Variable to Command Output

Use the following to set a variable to the output of a command at configure time. This is useful for instance if you can get the dependencies of a command from a script. See an example of that below

```cmake

add_custom_command(
	OUTPUT
		command_output_file
	COMMENT
		"Comment"
	COMMAND
		command_to_run
	DEPENDS
		${DEPENDS}
)

execute_process(}
	COMMAND
		command
	OUTPUT_VARIABLE
		DEPENDS
	WORKING_DIRECTORY
		dir
	OUTPUT_STRIP_TRAILING_WHITESPACE
)
```

where `command` should return a whitespace separated list of files.