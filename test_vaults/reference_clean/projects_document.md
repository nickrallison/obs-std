---
aliases:
  - Projects
tags:
  - work
  - "#interviewing"
bad_links:
---
# Projects Document

MapaRobo, May 2022 - Sept 2022, Software Engineering Intern
- Tools:
	- ROS, C++, Python
	- Electronics
- Accomplishments:
	- Fixed IMU Problem when no one else could
		- Robot yaw was drifting while in use. Did not seem to be happening while testing the compass while off the robot.
		- Tested the location we place the compass on the robot and it was magnetic causing the inaccurate readings
	- Made home made Kalman filter
		- Existing telemetry data was very noisy and jumpy, I made this to clean up the noise and jumpiness and get much more accurate location and telemetry data

Canis, Aug 2022 - Aug 2023 
- Tools:
	- ROS, C++
- Accomplishments
	- Got an inverse kinematics model working to get the ability to position the legs accurately
	- Got a walking model working to give it the ability to walk forward or turn

Signal Generator, December 2022

Qualcomm, May 2023 - Sept 2024, Electrical Engineering Intern, Design Team
- Tools:
	- SystemVerilog, C, C++, Bash, Python, Make, CMake, Rust, Linux
	- Synopsys Tools, Code Coverage, Verdi
- Skills: 
	- Power Optimized Design & Clock Gating with Calypto PowerPro
		- FP16 vs ARGB8 Power Analysis Results
			- Numbers didn't make sense, I thought it should have been double, but figured out it's doing an energy calculation and averaging over the time span
			- The reason it took longer was because FP16 was also getting limited bandwidth on the input
		- Demura Gating
			- Numbers looked like there was potential for gating
			- It turned out that a good portion of power was in a RAM block which could not be safely gated which negated a good chunk of the possible savings
	- Script and tool creation
		- PowerPro wrapper
			- Our existing PowerPro tool wasn't simple to use for new modules
			- I made a tool that just worked for modules that had not been ran before
		- Instance Finder
			- I had also made a tool to print all the signals and modules in a wave file
			- From this is was simple to print each module that
		- Documentation
			- These tools are not necessarily simple by default so I've created a lot of documentation for how to use them as well as the underlying tools they are built upon.
	- Verilog Test Optimization
		- Optimized Verilog run test by disabling unused register transactions
		- Disabling High Cost Assertions on common tests and migrated them to other tests on a weekly schedule
		- Implemented Multithreading on tests
		- Improved runtime by X%
	- CMake Migration
		- Spearheaded task to migrate the teams antiquated build system from a recursive Make system to a CMake System
		- Improved First time build time by **8x**
			- 1h 7min -> **X**
		- Improved Rebase and rebuild time by **X**%
	- Refactoring
		- Some was done on the fetch module
		- Some could not be done because it was deemed to be to breaking
			- It works so don't touch the stuff that's working
	- Working Fast
		- When given the opportunity to do so, I work quite fast
			- When I'm not waiting for a 12 hour synthesis or power analysis
			- When not waiting for a response
- Accomplishments
	- [[Wilo 3]]
	- CMake
		- Architecture of how we planned & structured it
		- Script Translation of Makefiles to CMake
		- Redid it all correctly, 
			- Parallel
			- dependencies specified
			- Non Polluting
- Challenges
	- Working on fetch was really difficult
		- Not well documented
		- Super old, not a lot of room for changes
		- Very Hard Coded
	- Code Cov for fetch
		- Not super well explained what signals do what
		- Those signals aren't documented either
		- The senior engineer was very organized but I was challenged in communicating effectively with him.
			- I usually couldn't ask all my questions at once or he wouldn't answer most of them
			- Or just didn't respond
	- Something I learned is that sometimes it's just easier to leave an adequate module as is without touching it when it can be changed and improved
		- The risk is usually not worth it
		- **Perfect is the enemy of good**

Home Linux Server Admin

RICE, Oct 2023
- Rust
- Numerical Node Voltage Circuit Simulator
- Can Simulate any RLC Circuit with any time varying or non time varying voltage source

Computer Algebra System, Oct 2023
- Rust
- Perform Arbitrary symbolic computations to arbitrary precision
- Used to perform arbitrary derivatives with no loss of accuracy on higher order terms

FSM Simulator

Note Linker

Obsidian Python Scripter