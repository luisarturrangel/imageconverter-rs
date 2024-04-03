![GitHub License](https://img.shields.io/github/license/luisarturrangel/imageconverter-rs)
# imageconverter-rs

Simple GUI application built with Rust that allows users to convert image files to different extensions.

## Features

- Convert image files to PNG, JPEG, BMP, WebP, ICO, and more.
- User-friendly interface.
- Cross-platform compatibility.

## Usage

1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Build the project using Cargo:
   ```
   cargo build --release
   ```
4. Run the executable:
   ```
   ./target/release/imageconverter-rs
   ```
5. Use the GUI to select the image file you want to convert and choose the desired output format.
6. Click the "Convert" button to initiate the conversion process.
7. Once the conversion is complete, the converted image will be saved in the specified location.

## Dependencies

- [Rust](https://www.rust-lang.org/)
- [Egui](https://github.com/emilk/egui)
- [Image](https://github.com/image-rs/image)
- [NFD](https://github.com/saurvs/nfd-rs)

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
