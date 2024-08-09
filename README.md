ESP32-C3 std template for those having trouble with the template provided by espressif.  
Some prerequisites:  
```
cargo install cargo-generate
cargo install ldproxy
```  
Also make sure that the Espressif C library (ESP-IDF) and paths are exported everytime you build a new project see official documentation below:  
https://docs.espressif.com/projects/esp-idf/en/stable/esp32/get-started/linux-macos-setup.html  
Use the command below to clone this repository.  
```
cargo generate https://github.com/VicStew/c3template
```  
This repository is configured to work with Wokwi simulation, so build and launch wokwi simulation, run if you have a dev board connected to your computer.
