---
aliases: 
tags:
  - coding
  - cmake
bad_links:
---
# CMake Example Template

## Command

```bash
cmake --workflow --preset [preset]
```

## CMakeLists

### Main
```cmake
cmake_minimum_required(VERSION 3.25)

project(main)
add_subdirectory([dir])
```

### Subdirectory

```cmake
cmake_minimum_required(VERSION 3.25)

project(exe)
add_executable(${PROJECT_NAME}
	SOURCES
		file1.c
)

target_include_directories(
	INTERFACE
		.
)
```

## CMakePresets

```json
{
	"version": 6,
	"cmakeMinimumRequried": {
		"major": 3,
		"minor": 25,
		"patch": 0
	},
	"configurePresets": [
		{
			"name": "default",
			"displayName": "Release Build",
			"description": "Release Build",
			"generator": "Ninja",
			"toolchainFile": "[toolchain.cmake]",
			"cacheVariables": [
				"CMAKE_BUILD_TYPE": "Release",
				"CMAKE_COMPILE_WARNING_AS_ERROR": "ON"
			],
			"environment": {
				"CC": "/path/to/gcc",
				"CXX": "/path/to/g++"
			}
		}
	],
	"buildPresets": [
		{
			"name": "default",
			"configuration": "Release",
			"configurationPreset": "default",
			"jobs": 32,
			"targets": ["install"]
		}
	],
	"workflowPreests": [
		{
			"name": "default",
			"steps": [
				{
					"type": "configure",
					"name": "default"
				},
				{
					"type": "build",
					"name": "default"
				}
			]
		}
	]
}
```

## Toolchain

```cmake
add_link_options(-Wl,--no-undefined)
...
```