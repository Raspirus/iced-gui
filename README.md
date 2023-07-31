# iced-gui
Raspirus is being merged to being a full-Rust application. This greatly helps with issues and dependencies. 
Tauri is a great tool, but by using its own Tauri CLI and having to communicate between two programming languages with two different styles and concepts, even simple projects can become quite difficult to manage. 
The [Iced GUI](https://github.com/iced-rs) has been chosen because of its great customization and usability. It is still a bit in beta-development and there is no official documentation, nonetheless it has some nice features, widgets, and It's not too difficult to implement for this project.

Here are the steps:
- [x] Create a central page controller
- [x] Start designing the individual pages with their states and messages
- [x] Implement a couple pages to start off the project
- [x] **(Critical)** Figure out communication between backend and frontend
- [x] Figure out how to pass parameters between pages
- [x] Figure out how to use components
- [x] **(Critical)** Implement the scanner
- [x] Finish an entire cycle: Path → Permission → Scan → Result
- [x] Style pages roughly using Iced
- [ ] Write test cases for frontend
- [ ] Perform accurate testing
- [ ] **(Critical)** Style frontend properly
- [ ] Test cross-compilation and building
- [ ] Merge dev branch into main
- [ ] Fix workflows and dependency dashboard
- [ ] Create release

Later on, the documentation will also need to be updated and rewritten to show the new programming language and framework

## :warning: This project is progressing slowly :warning:
Iced is not stable and at the time of writing I am not happy of how the app looks. Because of this, the merge to 100% Rust is on hold until the Iced library becomes more stable and finally publishes its documentation
