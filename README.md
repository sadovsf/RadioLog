# RadioLog
Small console based app for radioamateurs (HAM radio ethusiasts) to keep track of their connections easily and safely on their own PC. 
<img width="1774" alt="image" src="https://github.com/sadovsf/RadioLog/assets/2211533/ffb3dc07-74f0-4f31-af25-0afa6202a1ce">


## Goals
* Fast UI to render and navigate.
* Reasonably responsive UI
* All in one binary, no external libs needed for runtime
* Multiplatform
* Common storage for ease of migration (sqlite)


## What works
* Adding (a), editing (enter) and deleting (delete) "pins"
* Searching of coordinates based on open maps API (PageDown in name edit field of create dialog)
* Simple visualization of selected pins on map
* Zoomable map with panning around (num keys 8546 - wsad but on num keys)

## What does not work
* Ton of UX stuff
* Missing many UI elements
* No stats
* Not possible to keep multiple databases / folders
* probably much more....


## Notes
Project is in super early stage and at least for now it is intended for my own use and partially as interesting project to improve my Rust knowladge. 
Feel free to try it out but no guarantees are provided ie use on your own risk of data loss.
