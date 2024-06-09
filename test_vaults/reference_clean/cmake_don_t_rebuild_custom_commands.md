---
aliases:
tags:
  - coding
  - cmake
bad_links:
---
# CMake Don't Rebuild Custom Commands

If you want non-compiled targets to run when you build a project. You can add a dummy_target, and add the output of the custom commands as file dependencies. That way they will only be rebuilt if out of date.

**CMakeLists.txt**
```cmake
cmake_minimum_required(VERSION 3.25)

project(dummy_target)
add_executable(${PROJECT_NAME}
	SOURCES
		main.c
)

add_dependancies(${PROJECT_NAME}
	command_1_target
)

add_custom_target(command_1_target
	DEPENDS
		command_output_file
)

add_custom_command(
	OUTPUT
		command_output_file
	COMMENT
		"Comment"
	COMMAND
		command_to_run
	DEPENDS
		Command
		Dependancies
)
```

**main.c**
```c
int main() {
	return 0;
}
```

